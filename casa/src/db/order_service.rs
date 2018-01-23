use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, OrderSettlementBuilder, Id};

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    db_helper: T
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { db_helper: db_helper }
    }

    pub fn place_order(&mut self, owner_id: Id, order: &Order) -> Result<Order, PostgresHelperError> {
        let sell_asset_units = order.sell_asset_units as i64;
        let buy_asset_units = order.buy_asset_units as i64;

        let execute_result = self.db_helper.execute(INSERT_NEW_ORDER_SQL, &[
            &order.buy_asset_type.to_string(),
            &order.buy_asset_denom.to_string(),
            &order.sell_asset_type.to_string(),
            &order.sell_asset_denom.to_string(),
            &order.unique_id,
            &owner_id,
            &sell_asset_units,
            &buy_asset_units,
            &order.expires_at.naive_utc()
        ]);

        match execute_result {
            Ok(rows) =>  {
                let new_order = try!(self.get_order_by_unique_id(owner_id, &order.unique_id));
                new_order.ok_or(PostgresHelperError::new("Failed to retrieve order after placing it."))
            },
            Err(err) => {
                Err(PostgresHelperError::new(&format!("Failed to execute order placement function: {:?}", err)))
            }
        }
    }

    pub fn cancel_order(&mut self, order_id: Id) -> Result<Option<Order>, PostgresHelperError> {
        try!(self.db_helper.execute(CANCEL_ORDER_SQL, &[]));
        self.get_order_by_id(order_id)
    }

    pub fn get_all_active_orders(&mut self) -> Result<Vec<Order>, PostgresHelperError> {
        match self.db_helper.query(SELECT_ALL_ACTIVE_ORDERS_SQL, &[]) {
            Ok(orders) => Ok(orders),
            Err(error) => {
                Err(PostgresHelperError::new(&format!("Failed to get orders: {:?}", error)))
            }
        }
    }

    pub fn get_all_active_orders_by_user(&mut self, email_address: &str) -> Result<Vec<Order>, PostgresHelperError> {
        match self.db_helper.query(SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL, &[&email_address]) {
            Ok(orders) => Ok(orders),
            Err(error) => {
                Err(PostgresHelperError::new(&format!("Failed to get orders for user: {:?}", error)))
            }
        }
    }

    pub fn get_order_settlement_status(&mut self, order_id: Id) 
        -> Result<Option<OrderSettlement>, PostgresHelperError> {
        let settlement_result = self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id]);
        let settlement: OrderSettlementBuilder;

        if let Some(s) = try!(settlement_result).pop() {
            settlement = s;
        } else {
            return Ok(None);
        }

        let settlement_id = settlement.id.unwrap();

        let mut orders_in_settlement_result: Vec<OrderSettlementBuilder> = 
            try!(self.db_helper.query(SELECT_ORDERS_IN_SETTLEMENT_SQL,
            &[&settlement_id]));

        let order_settlement_builder: OrderSettlementBuilder;

        if let Some(s) = orders_in_settlement_result.pop() {
            order_settlement_builder = s;
        } else {
            return Ok(None);
        }

        let mut order_result: Vec<Order> = try!(self.db_helper.query(SELECT_ORDERS_IN_SETTLEMENT_SQL,
            &[&settlement_id]));

        let buying_order: Order;
        let selling_order: Order;
        if order_result.len() != 2 {
            let error_message = format!("Settlement should have two orders, but got {}",
                order_result.len());
            return Err(PostgresHelperError::new(&error_message));
        }
        buying_order = order_result.pop().unwrap();
        selling_order = order_result.pop().unwrap();

        let settlement = order_settlement_builder.build(buying_order, selling_order);

        Ok(Some(settlement))
    }

    pub fn get_order_by_id(&mut self, order_id: Id) -> Result<Option<Order>, PostgresHelperError> {
        match self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id]) {
            Ok(mut orders) => Ok(orders.pop()),
            Err(error) => {
                Err(PostgresHelperError::new(&format!("Failed to get order: {:?}", error)))
            }
        }
    }

    pub fn get_order_by_unique_id(&mut self, owner_id: Id, unique_id: &str) -> Result<Option<Order>, PostgresHelperError> {
        match self.db_helper.query(SELECT_ORDER_UNIQUE_ID_SQL, &[&owner_id, &unique_id]) {
            Ok(mut orders) => Ok(orders.pop()),
            Err(error) => {
                Err(PostgresHelperError::new(&format!("Failed to get order: {:?}", error)))
            }
        }
    }
}

const INSERT_NEW_ORDER_SQL: &'static str = "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_ORDER_UNIQUE_ID_SQL: &'static str =  
    "SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         account_owner owners, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE orders.owner_id = owners.id AND
          buy_asset_type.id = orders.buy_asset_type_id AND
          sell_asset_type.id = orders.sell_asset_type_id AND
          owners.id = $1 AND
          orders.unique_id = $2";

const SELECT_ORDER_BY_ID_SQL: &'static str = "
    SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         account_owner owners, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE orders.owner_id = owners.id AND
          buy_asset_type.id = orders.buy_asset_type_id AND
          sell_asset_type.id = orders.sell_asset_type_id AND
          orders.id = $1";

const CANCEL_ORDER_SQL: &'static str = "
    UPDATE asset_orders SET status = 'user_cancelled' 
    WHERE status = 'active' AND expires_at < (now() at time zone 'utc') AND id = $1";

const UPDATE_ORDERS_EXPIRED_SQL: &'static str = "
    UPDATE asset_orders SET status = 'expires' 
    WHERE status = 'active' AND expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         account_owner owners, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE orders.status = 'active' AND orders.expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         account_owner owners, 
         users users, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE orders.owner_id = owners.id AND
          buy_asset_type.id = orders.buy_asset_type_id AND
          sell_asset_type.id = orders.sell_asset_type_id AND 
          users.id = owners.user_id AND
          users.email_address = $1";

const SELECT_ORDER_SETTLEMENT_SQL: &'static str = "
    SELECT settlements.*, settlements.id as order_settlement_id FROM 
        asset_order orders,
        order_settlement settlements
    WHERE 
        orders.id = $1";

const SELECT_ORDERS_IN_SETTLEMENT_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         asset_order cp_order,
         account_owner owners, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type,
         order_settlement settlements
    WHERE 
        orders.settlement_id = $1
";

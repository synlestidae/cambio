use db::{PostgresHelper, CambioError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, OrderSettlementBuilder, Id};

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    db_helper: T,
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { db_helper: db_helper }
    }

    pub fn place_order(
        &mut self,
        owner_id: Id,
        order: &Order,
    ) -> Result<Order, CambioError> {
        let sell_asset_units = order.sell_asset_units as i64;
        let buy_asset_units = order.buy_asset_units as i64;

        let execute_result = self.db_helper.execute(
            INSERT_NEW_ORDER_SQL,
            &[
                &order.buy_asset_type,
                &order.buy_asset_denom,
                &order.sell_asset_type,
                &order.sell_asset_denom,
                &order.unique_id,
                &owner_id,
                &sell_asset_units,
                &buy_asset_units,
                &order.expires_at.naive_utc(),
            ],
        );

        let rows = try!(execute_result);
        let new_order = try!(self.get_order_by_unique_id(owner_id, &order.unique_id));
        let mut retrieve_error = CambioError::shouldnt_happen(
            "Your order could not be retrieved. Check the Orders page and try again", 
            "get_order_by_unique_id returned None");
        new_order.ok_or(retrieve_error)
    }

    pub fn cancel_order(&mut self, order_id: Id) -> Result<Option<Order>, CambioError> {
        try!(self.db_helper.execute(CANCEL_ORDER_SQL, &[&order_id]));
        self.get_order_by_id(order_id)
    }

    pub fn get_all_active_orders(&mut self) -> Result<Vec<Order>, CambioError> {
        let active_orders = try!(self.db_helper.query(SELECT_ALL_ACTIVE_ORDERS_SQL, &[]));
        Ok(active_orders)
    }

    pub fn get_all_active_orders_by_user(
        &mut self,
        email_address: &str,
    ) -> Result<Vec<Order>, CambioError> {
        let orders = try!(self.db_helper.query(
            SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL,
            &[&email_address],
        ));
        Ok(orders)
    }

    pub fn get_order_settlement_status(
        &mut self,
        order_id: Id,
    ) -> Result<Option<OrderSettlement>, CambioError> {
        let settlement_result = self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id]);
        let settlement: OrderSettlementBuilder;

        if let Some(s) = try!(settlement_result).pop() {
            settlement = s;
        } else {
            return Ok(None);
        }

        let settlement_id = settlement.id.unwrap();

        let mut orders_in_settlement_result: Vec<OrderSettlementBuilder> =
            try!(self.db_helper.query(
                SELECT_ORDERS_IN_SETTLEMENT_SQL,
                &[&settlement_id],
            ));

        let order_settlement_builder: OrderSettlementBuilder;

        if let Some(s) = orders_in_settlement_result.pop() {
            order_settlement_builder = s;
        } else {
            return Ok(None);
        }

        let mut order_result: Vec<Order> = try!(self.db_helper.query(
            SELECT_ORDERS_IN_SETTLEMENT_SQL,
            &[&settlement_id],
        ));

        let buying_order: Order;
        let selling_order: Order;
        if order_result.len() != 2 {
            let sys_message = format!("Expected 2 orders, got {}", order_result.len());
            return Err(CambioError::shouldnt_happen("Failed to match the orders in settlement. ", &sys_message));
        }
        buying_order = order_result.pop().unwrap();
        selling_order = order_result.pop().unwrap();

        let settlement = order_settlement_builder.build(buying_order, selling_order);

        Ok(Some(settlement))
    }

    pub fn get_order_by_id(&mut self, order_id: Id) -> Result<Option<Order>, CambioError> {
        let mut orders = try!(self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id])); 
        Ok(orders.pop())
    }

    pub fn get_order_by_unique_id(
        &mut self,
        owner_id: Id,
        unique_id: &str,
    ) -> Result<Option<Order>, CambioError> {
        let order_result = self.db_helper.query(
            SELECT_ORDER_UNIQUE_ID_SQL,
            &[&owner_id, &unique_id],
        );

        let mut order_list = try!(order_result);
        Ok(order_list.pop())
    }


    // ALL METHODS MUST BE IMMUNE TO REPLAY ATTACKS

    pub fn settle_two_orders(&mut self, buying_crypto_order: &Order, selling_order: &Order) 
        -> Result<OrderSettlement, CambioError> {
            // steps to settle 
            // -1 check that orders aren't settled already
            // 0 check that past transactions haven't already been made
            // 1 retrieve two orders from DB 
            // 2 check orders both match - they must be what user is looking for
            // 3 retrieve the monetary account for the fiat-currency receiver
            // 4 retrieve the ethereum account for the crypto-currency receiver 
            // 5 check that both accounts have sufficient funds
            // 6 insert settlement into DB
            // 7 transfer fiat funds from crypto-currency receiver to holding account
            // 8 mark settlement as pending ethereum transaction
            // 9 perform ethereum transaction
            // 10 check ethereum transaction has been confirmed, and do one API lookup
            // 11 mark settlement as ethereum confirmed, pending fiat fund transfer
            // 12 transfer fiat funds from holding account to fiat currency receiver
            // 13 mark settlement as finished
        unimplemented!()
    }
}

const INSERT_NEW_ORDER_SQL: &'static str = "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_ORDER_UNIQUE_ID_SQL: &'static str = "SELECT 
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
    UPDATE asset_order SET status = 'user_cancelled' 
    WHERE status = 'active' AND expires_at > (now() at time zone 'utc') AND id = $1";

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

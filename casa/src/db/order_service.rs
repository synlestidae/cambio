use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id};

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    db_helper: T
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { db_helper: db_helper }
    }

    pub fn place_order(&mut self, owner_id: Id, order: &Order) -> Result<Order, PostgresHelperError> {
        let ttl_milliseconds = ((order.expires_at.timestamp() - Utc::now().timestamp()) * 1000) as u32;

        let sell_asset_units = order.sell_asset_units as i64;
        let buy_asset_units = order.buy_asset_units as i64;

        println!("Running stored procedure for new order");

        let execute_result = self.db_helper.execute(INSERT_NEW_ORDER_SQL, &[
            &order.buy_asset_type.to_string(),
            &order.buy_asset_denom.to_string(),
            &order.sell_asset_type.to_string(),
            &order.sell_asset_denom.to_string(),
            &order.unique_id,
            &owner_id,
            &sell_asset_units,
            &buy_asset_units,
            &ttl_milliseconds,
        ]);

        println!("Ran the stored procedure!");

        match execute_result {
            Ok(_) =>  {
                println!("Got some rows");
                let new_order = try!(self.get_order_by_unique_id(owner_id, &order.unique_id));
                println!("Got the order!");
                new_order.ok_or(PostgresHelperError::new("Failed to retrieve order after placing it."))
            },
            Err(err) => {
                Err(PostgresHelperError::new(&format!("Failed to execute order placement function: {:?}", err)))
            }
        }
    }

    pub fn cancel_order(order_id: Id) -> Result<Order, PostgresHelperError> {
        unimplemented!();
    }

    pub fn get_all_active_orders() -> Result<Vec<Order>, PostgresHelperError> {
        unimplemented!();
    }

    pub fn get_user_active_orders(email_address: &str) -> Result<Vec<Order>, PostgresHelperError> {
        unimplemented!();
    }

    pub fn get_order_settlement_status(order_id: Id) 
        -> Result<Option<OrderSettlement>, PostgresHelperError> {
        unimplemented!();
    }

    pub fn get_order_by_unique_id(&mut self, owner_id: Id, unique_id: &str) -> Result<Option<Order>, PostgresHelperError> {
        match self.db_helper.query(SELECT_ORDER_UNIQUE_ID_SQL, &[&owner_id, &unique_id]) {
            Ok(mut orders) => Ok(orders.pop()),
            Err(error) => Err(PostgresHelperError::new(&format!("Failed to get order: {}", error)))
        }
    }
}

const INSERT_NEW_ORDER_SQL: &'static str = "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_ORDER_UNIQUE_ID_SQL: &'static str =  
    "SELECT 
        *, 
        asset_order.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.asset_denom AS buy_asset_denom,  
    FROM asset_order,
         account_owner, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE asset_order.owner_id = owner.id AND
          buy_asset_type.id = asset_order.buy_asset_type_id AND
          sell_asset_type.id = asset_order.sell_asset_type_id
";

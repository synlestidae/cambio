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

    // wow this is a wee bit complicated eh
    pub fn place_order(&mut self, order: &Order) -> Result<Order, PostgresHelperError> {
        let ttl_milliseconds = 
            (order.expires_at.timestamp() - Utc::now().timestamp()) * 1000;

        let sell_asset_units = order.sell_asset_units as i64;
        let buy_asset_units = order.buy_asset_units as i64;

        let execute_result = self.db_helper.execute(INSERT_NEW_ORDER_SQL, &[
            &order.buy_asset_type.to_string(),
            &order.buy_asset_denom.to_string(),
            &order.sell_asset_type.to_string(),
            &order.sell_asset_denom.to_string(),
            &order.unique_id,
            &order.owner_id,
            &sell_asset_units,
            &buy_asset_units,
            &ttl_milliseconds,
        ]);

        match execute_result {
            Ok(_) =>  {
               unimplemented!() 
            },
            Err(err) => {
                Err(PostgresHelperError::new(&format!("Failed to execute order placement function: {}", err)))
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
}

const INSERT_NEW_ORDER_SQL: &'static str = "
    DO $$
        DECLARE sell_asset_type_id_var INTEGER;
        DECLARE buy_asset_type_id_var INTEGER;
    BEGIN

    SELECT id INTO buy_asset_type_id FROM get_asset_id($1, $2);
    SELECT id INTO sell_asset_type_id FROM get_asset_id($3, $4);

    INSERT INTO asset_order(owner_id, sell_asset_units, buy_asset_units, sell_asset_type_id,
        buy_asset_type_id, ttl_milliseconds) 
     VALUES($5, $6, $7, $8, sell_asset_type_id_var, buy_asset_type_id_var, $9);

    END $$;";

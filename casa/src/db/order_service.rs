use db::{PostgresHelper, PostgresHelperError};
use domain::Order;

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    db_helper: T
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { db_helper: db_helper }
    }

    // wow this is a wee bit complicated eh
    pub fn place_order(order: &Order) -> Result<Order, PostgresHelperError> {
        unimplemented!();
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
        unimplemented!();
        -> Result<Option<OrderSettlement>, PostgresHelperError> {
    }
}

use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id, EthereumAccountDetails};

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn register_transaction(&mut self, 
        account: &EthereumAccountDetails, 
        password: String) -> Result<EthereumOutboundTransaction, PostgresHelperError> {
        let private_key = account.decrypt_private_key(password).unwrap();
        unimplemented!()
    }

    pub get_gas_price_szabo_(&mut self) -> u64 {
        unimplemented!()
    }
}
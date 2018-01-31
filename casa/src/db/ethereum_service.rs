use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id, EthereumAccountDetails, EthereumOutboundTransaction};
use web3;
use web3::futures::Future;
use hex;

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn register_transaction(&mut self, 
        account: &EthereumAccountDetails, 
        password: String) -> Result<EthereumOutboundTransaction, PostgresHelperError> {

        let private_key = account.decrypt_private_key(password).unwrap();
        let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = web3::Web3::new(http);
        let mut address_bytes: [u8; 20] = [0; 20];
        let hex_addr = "A990F82d33Fd19C3872dc12c588A66224b9330A6";
        for (i, b) in hex::decode(hex_addr).unwrap().into_iter().enumerate() {
            address_bytes[i] = b;
        }
        let balance = web3.eth().balance(web3::types::H160(address_bytes), None).wait().unwrap();
        unimplemented!()
    }

    pub fn get_gas_price_szabo(&mut self) -> Result<u64, PostgresHelperError> {
        let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = web3::Web3::new(http);
        web3.eth().gas_price().wait().unwrap();
        unimplemented!()
    }
}

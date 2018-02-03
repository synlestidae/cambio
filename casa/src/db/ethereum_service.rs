use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id, EthereumAccountDetails, EthereumOutboundTransaction};
use web3;
use web3::futures::Future;
use hex;
use web3::types::{H160, H512, Bytes};
use std::str::FromStr;

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn new(db_helper: T) -> Self {
        Self {
            db_helper: db_helper
        }
    }
    pub fn register_transaction(&mut self, 
        account: &EthereumAccountDetails, 
        password: String,
        amount_wei: u64,
        destination_address: H512) -> Result<EthereumOutboundTransaction, PostgresHelperError> {
        

        let private_key = account.decrypt_private_key(password).unwrap();
        let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = web3::Web3::new(http);
        /*let mut private_key_bytes: [u8; 32] = [0; 32];
        for (i, b) in hex::decode(private_key).unwrap().into_iter().enumerate() {
            private_key_bytes[i] = b;
        }*/
        let transaction = EthereumOutboundTransaction {
            id: None,
            nonce: 0,
            gas_price: 18000000000,
            gas_limit: 18000000000,
            to_address: "0x9f23fedfa2ce3a321f20f6a95d0c2cbabb5876dd".to_owned(),
            from_address: "0x927B18DD62B0500Cfed48815D1a613e2f1167903".to_owned(),
            hash: String::new(),
            value: 1000000000,
            signature: None,
            transaction_block_id: None,
            unique_id: "test_boi".to_owned()
        };

        let raw_transaction = transaction.get_web3_transaction(&H512::from_str(&private_key).unwrap(), &mut web3.eth()).unwrap();
        let raw_bytes = Bytes::from(raw_transaction);
        web3.eth().send_raw_transaction(raw_bytes).wait().unwrap();
        unimplemented!();
    }

    pub fn get_gas_price_szabo(&mut self) -> Result<u64, PostgresHelperError> {
        let (_eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
        let web3 = web3::Web3::new(http);
        web3.eth().gas_price().wait().unwrap();
        unimplemented!()
    }
}

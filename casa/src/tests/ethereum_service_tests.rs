use domain::{EthAccount, EthereumOutboundTransaction};
use db::{PostgresHelperImpl, EthereumService};
use tests::test_utils;
use web3::types::{H160, H256};
use std::str::FromStr;

#[test]
pub fn test_transaction() {
    let mut eth_service = get_service();
    println!("Do account");
    let account = EthAccount::new("9f23fedfa2ce3a321f20f6a95d0c2cbabb5876dd",
        "77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b".to_owned(),
        "iliketurtles".to_owned());
    println!("Register boi");
    let private_key = H256::from_str("0x77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b").unwrap();
    println!("Done private");
    eth_service.register_transaction(&account, 
        "iliketurtles".to_owned(), 
        100000, 
        private_key);
}

#[allow(dead_code)]
pub fn get_service() -> EthereumService<PostgresHelperImpl> {
    EthereumService::new(test_utils::get_db_helper(), "http://localhost:8080")
}

use domain::{EthAccount, EthereumOutboundTransaction};
use db::{PostgresHelperImpl, EthereumService, UserRepository};
use tests::test_utils;
use web3::types::{H160, H256};
use std::str::FromStr;

#[test]
pub fn test_transaction() {
    println!("Starting test");
    /*let pub_key = H160::from_str("0x9f23fedfa2ce3a321f20f6a95d0c2cbabb5876dd").unwrap();
    let priv_key = H256::from_str("0x77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b").unwrap();*/
    let mut service = get_service();
    let mut repo = get_repo();
    println!("Registering user");
    //repo.register_user("jerry@springer.com", "iloveturtles".to_owned()).unwrap();
    println!("Getting a new account");
    let account = service.new_account("jerry@springer.com", "iliketurtles".to_owned()).unwrap();
    println!("Making the transaction");
    service.register_transaction(&account, 
        "iliketurtles".to_owned(), 
        10000000, 
        1000000, 
        H160::from_str("0x927B18DD62B0500Cfed48815D1a613e2f1167903").unwrap(),
        "transaction123");
}

#[allow(dead_code)]
pub fn get_service() -> EthereumService<PostgresHelperImpl> {
    EthereumService::new(test_utils::get_db_helper(), "/Users/mate/work/cambio/eth_test/data/geth.ipc")
}

#[allow(dead_code)]
pub fn get_repo() -> UserRepository<PostgresHelperImpl> {
    UserRepository::new(test_utils::get_db_helper())
}

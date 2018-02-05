use domain::{EthAccount, EthereumOutboundTransaction};
use db::{PostgresHelperImpl, EthereumService, UserRepository};
use tests::test_utils;
use web3::types::{H160, H256};
use std::str::FromStr;

#[test]
pub fn test_transaction() {
    println!("Starting test");
    let mut service = get_service();
    let mut repo = get_repo();
    println!("Registering user");
    repo.register_user("jerry@thesmithfamily.com", "iloveturtles".to_owned()).unwrap();
    println!("Getting a new account");
    let account = service.new_account("jerry@thesmithfamily.com", "iliketurtles".to_owned()).unwrap();
    println!("Making the transaction");
    service.register_transaction(&account, 
        "iliketurtles".to_owned(), 
        10000000, 
        1000000, 
        H160::from_str("0x927B18DD62B0500Cfed48815D1a613e2f1167903").unwrap(),
        "transaction123").unwrap();
}

#[allow(dead_code)]
pub fn get_service() -> EthereumService<PostgresHelperImpl> {
    EthereumService::new(test_utils::get_db_helper(), "/Users/mate/work/cambio/eth_test/data/geth.ipc")
}

#[allow(dead_code)]
pub fn get_repo() -> UserRepository<PostgresHelperImpl> {
    UserRepository::new(test_utils::get_db_helper())
}

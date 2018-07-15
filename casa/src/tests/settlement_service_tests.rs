use chrono::prelude::*;
use db;
use domain;
use repositories;
use repository;
use repository::*;
use services;
use tests::order_utils::*;
use uuid;
use web3::types::U256;

#[test]
fn refuses_settlement_no_eth_balance() {
    let mut settlement_service = services::SettlementService::new(
        "http://localhost:8081",
    );
    let (order1, order2) = quick_order(
        "ricky@gervais.com",
        "karl@pilkington.com",
        100000,
        200 * 100,
        100000,
        200 * 100,
    );
    quick_credit("ricky@gervais.com", 200 * 100);
    let ricky = get_user("ricky@gervais.com");

    let mut settlement = settlement_service
        .create_settlement(ricky.id.unwrap(), &order1, &order2)
        .unwrap();

    assert_eq!(
        settlement.settlement_status,
        domain::SettlementStatus::WaitingEthCredentials
    );

    let result = settlement_service.begin_eth_transfer(
        settlement.id.unwrap(),
        "981upr983ucn982qr2349t9y34%tp9q83tup983q4",
        "dohnut123".to_owned(),
        U256([0, 0, 0, 21000000000000])
    );

    assert!(result.is_err());
}

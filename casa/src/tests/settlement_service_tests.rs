use chrono::prelude::*;
use db;
use domain;
use repository;
use repository::*;
use services;
use tests::order_utils::*;
use tests::test_utils::*;
use uuid;
use web3::types::U256;

#[test]
fn refuses_settlement_no_eth_balance() {
    use std::env;
    let (eloop, web3) = get_web3();
    let mut settlement_service = services::SettlementService::new(web3);
    const KARL: &'static str = "karl@orangeheadedbuffoon.com";
    const RICKY: &'static str = "ricky@gangstermail.com";
    let (order1, order2) = quick_order(
        RICKY,
        KARL,
        100000,
        200 * 100,
        100000,
        200 * 100,
    );
    quick_credit(RICKY, 200 * 100);
    let ricky = get_user(RICKY);
    get_user(KARL);

    let mut settlement = settlement_service
        .create_settlement(
            &mut get_db_connection(),
            ricky.id.unwrap(),
            &order1,
            &order2,
        )
        .unwrap();

    assert_eq!(
        settlement.settlement_status,
        domain::SettlementStatus::WaitingEthCredentials
    );

    let result = settlement_service.begin_eth_transfer(
        &mut get_db_connection(),
        settlement.id.unwrap(),
        "981upr983ucn982qr2349t9y34%tp9q83tup983q4",
        "dohnut123".to_owned(),
        U256([0, 0, 0, 21000000000000]),
    );

    drop(settlement_service);
    drop(eloop);

    assert!(result.is_err());
}

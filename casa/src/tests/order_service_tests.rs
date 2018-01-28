use db::{PostgresHelperImpl, UserRepository, OrderService};
use time::Duration;
use tests::user_repository_tests::get_repository;
use domain::{Order, AssetType, Denom, OrderStatus};
use chrono::prelude::*;
use std::process;
use tests::test_utils::*;
use std::clone::Clone;
use domain;
use db;

#[allow(dead_code)]
fn get_service() -> OrderService<PostgresHelperImpl> {
    OrderService::new(get_db_helper())
}

#[test]
fn test_operations_one_order() {
    run_test(|| {
        let mut user_repository = get_repository();
        let mut order_service = get_service();

        user_repository.register_user("jacinda@newzealand.co.nz", "secret123".to_owned());
        let owner_id = user_repository
            .get_owner_id_by_email_address("jacinda@newzealand.co.nz")
            .unwrap();

        let mut order = Order {
            id: None,
            unique_id: "bE9WO$h&Q#YQ%s@7mF2Zq9ecgB6XO)dC".to_owned(),
            sell_asset_units: 30000,
            buy_asset_units: 2000,
            sell_asset_type: AssetType::BTC,
            sell_asset_denom: Denom::Sat,
            buy_asset_type: AssetType::NZD,
            buy_asset_denom: Denom::Cent,
            expires_at: Utc::now() + Duration::minutes(10),
            status: OrderStatus::Active
        };

        let placed_order = order_service.place_order(owner_id, &order).unwrap();
        let mut placed_order_1 = order_service
            .get_order_by_unique_id(owner_id, &order.unique_id)
            .unwrap()
            .unwrap();
        let mut placed_order_2 = order_service
            .get_order_by_id(placed_order.id.unwrap())
            .unwrap()
            .unwrap();
        let all_orders = order_service
            .get_all_active_orders_by_user("jacinda@newzealand.co.nz")
            .unwrap();
        assert_eq!(placed_order, all_orders[0]);
        assert_eq!(placed_order_1, placed_order_2);
        placed_order_1.id = None;
        placed_order_2.id = None;
        assert_eq!(order, placed_order_1);
        assert_eq!(order, placed_order_2);

        order_service.cancel_order(placed_order.id.unwrap()).unwrap();
        let cancelled_order = order_service.get_order_by_id(placed_order.id.unwrap()).unwrap().unwrap();
        assert_eq!(OrderStatus::UserCancelled, cancelled_order.status);
        assert!(order_service.place_order(owner_id, &order).is_err());
    });
}

#[test]
fn test_two_orders_settled() {
    let mut user_repository = get_repository();
    let mut order_service = get_service();
    let mut account_repository = db::AccountRepository::new(get_db_helper());

    // contras want to buy ethereum from US gov
    // first credit the contras account with NZD
    
    user_repository.register_user("president@usa.gov", "secret123".to_owned()).unwrap();
    user_repository.register_user("contras@nicaragua.com", "odiamoselcomunismo".to_owned()).unwrap();

    let mut payment_builder = domain::PaymentBuilder::new(AssetType::NZD, 
        domain::Denom::Cent,
        domain::PaymentMethod::NZBankDeposit, 
        domain::PaymentVendor::Poli);

    let payment = payment_builder.transaction_details("129870c0-aebb-40f0-bed6-c81b1229d96e", 
        Utc::now(),
        1000 * 100).unwrap(); // 1000 bucks


    let mut payment_repo = db::PaymentRepository::new(get_db_helper(), 
        account_repository.clone(), 
        user_repository.clone());

    payment_repo.register_credit_payment("contras@nicaragua.com", &payment).unwrap();
    let contras_owner_id = user_repository
        .get_owner_id_by_email_address("contras@nicaragua.com")
        .unwrap();

    let president_owner_id = user_repository
        .get_owner_id_by_email_address("president@usa.gov")
        .unwrap();

    // pretend price of 1 ETH = 900 NZD
    let sell_crypto_order = Order {
        id: None,
        unique_id: "aT1wB#h&Q#YQ%s@7mF2Zq9ecgB7l3)%0".to_owned(),
        sell_asset_units: 1000000, // 1 eth in Szabo
        buy_asset_units: 900 * 1000, // 900 dollars in cents
        sell_asset_type: AssetType::ETH,
        sell_asset_denom: Denom::Szabo,
        buy_asset_type: AssetType::NZD,
        buy_asset_denom: Denom::Cent,
        expires_at: Utc::now() + Duration::minutes(10),
        status: OrderStatus::Active
    };

    // pretend price of 1 ETH = 900 NZD
    let buy_crypto_order = Order {
        id: None,
        unique_id: "Tq2b40*(2n(2n89p214n(Yhn32rf89hv".to_owned(),
        sell_asset_units: 900 * 1000, // 1 eth in Szabo
        buy_asset_units: 1000000, // 900 dollars in cents
        sell_asset_type: AssetType::NZD,
        sell_asset_denom: Denom::Cent,
        buy_asset_type: AssetType::ETH,
        buy_asset_denom: Denom::Szabo,
        expires_at: Utc::now() + Duration::minutes(10),
        status: OrderStatus::Active
    };

    let contra_order = order_service.place_order(contras_owner_id, &sell_crypto_order).unwrap();
    let president_order = order_service.place_order(president_owner_id, &buy_crypto_order).unwrap();
    order_service.settle_two_orders(&contra_order, &president_order).unwrap();
}

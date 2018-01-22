use db::{PostgresHelperImpl, UserRepository, OrderService};
use time::Duration;
use tests::user_repository_tests::get_repository;
use domain::{Order, AssetType, Denom};
use chrono::prelude::*;
use std::process;
use tests::test_utils::*;

#[test]
fn test_places_one_order() {
    run_test(|| {
        let mut user_repository = get_repository();
        let mut order_service = get_service();

        user_repository.register_user("jacinda@newzealand.co.nz", "secret123".to_owned());
        let owner_id = 
            user_repository.get_owner_id_by_email_address("jacinda@newzealand.co.nz").unwrap();
        let mut order = Order {
            id: None,
            unique_id: "bE9WO$h&Q#YQ%s@7mF2Zq9ecgB6XO)dC".to_owned(),
            sell_asset_units: 30000,
            buy_asset_units: 2000,
            sell_asset_type: AssetType::BTC,
            sell_asset_denom: Denom::Sat,
            buy_asset_type: AssetType::NZD,
            buy_asset_denom: Denom::Cent,
            expires_at: Utc::now() + Duration::minutes(10)
        };
        order_service.place_order(owner_id, &order).unwrap();
        let mut placed_order = order_service.get_order_by_unique_id(owner_id, &order.unique_id).unwrap().unwrap();
        placed_order.id = None;
        assert_eq!(order, placed_order);
    });
}

fn get_service() -> OrderService<PostgresHelperImpl> {
    OrderService::new(get_db_helper())
}

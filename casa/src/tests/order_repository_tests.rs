use domain;
use repositories::{UserRepository, OrderRepository};
use repository::Repository;
use repository;
use tests::get_db_helper;

#[test]
fn test_creates_ethereum_order() {
    let mut user = domain::User::new_register("rick@sanchez.space", "m000000rtyburp".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut original_order = domain::Order::buy_szabo(user.owner_id.unwrap(), 250000, 31929, 10);
    let mut order = order_repo.create(&original_order).unwrap();
    order = order_repo.read(&repository::UserClause::Id(order.id.unwrap()))
        .unwrap()
        .pop()
        .unwrap();
    order.id = None;

    assert_eq!(original_order.unique_id, order.unique_id);
    assert_eq!(original_order.sell_asset_units, order.sell_asset_units);
    assert_eq!(original_order.buy_asset_units, order.buy_asset_units);
    assert_eq!(original_order.sell_asset_type, order.sell_asset_type);
    assert_eq!(original_order.sell_asset_denom, order.sell_asset_denom);
    assert_eq!(original_order.buy_asset_type, order.buy_asset_type);
    assert_eq!(original_order.buy_asset_denom, order.buy_asset_denom);
    assert_eq!(original_order.status, order.status);
}

#[test]
fn test_creates_monetary_order() {
    let mut user = domain::User::new_register("morty@smith.family", "awwwwjeezrick".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut original_order = domain::Order::sell_szabo(user.owner_id.unwrap(), 31929, 250000, 10);
    let mut order = order_repo.create(&original_order).unwrap();
    order = order_repo.read(&repository::UserClause::Id(order.id.unwrap()))
        .unwrap()
        .pop()
        .unwrap();
    order.id = None;

    assert_eq!(original_order.unique_id, order.unique_id);
    assert_eq!(original_order.sell_asset_units, order.sell_asset_units);
    assert_eq!(original_order.buy_asset_units, order.buy_asset_units);
    assert_eq!(original_order.sell_asset_type, order.sell_asset_type);
    assert_eq!(original_order.sell_asset_denom, order.sell_asset_denom);
    assert_eq!(original_order.buy_asset_type, order.buy_asset_type);
    assert_eq!(original_order.buy_asset_denom, order.buy_asset_denom);
    assert_eq!(original_order.status, order.status);
}

#[test]
fn updates_order() {
    let mut user = domain::User::new_register("beth@smith.family", "jerrygetajob".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut original_order = domain::Order::sell_szabo(user.owner_id.unwrap(), 31929, 250000, 10);
    let mut order = order_repo.create(&original_order).unwrap();

    order.status = domain::OrderStatus::Deleted;
    order = order_repo.update(&order).unwrap();
    assert_eq!(domain::OrderStatus::Deleted, order.status);
}

fn deletes_order() {
    let mut user = domain::User::new_register("beth@smith.family", "jerrygetajob".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut original_order = domain::Order::sell_szabo(user.owner_id.unwrap(), 31929, 250000, 10);
    let mut order = order_repo.create(&original_order).unwrap();
    order = order_repo.delete(&order).unwrap();
    assert_eq!(order.status, domain::OrderStatus::Deleted);
    //assert_eq!(domain::OrderStatus::Deleted, order.status);
}

#[test]
fn cannot_delete_order() {
    let mut user = domain::User::new_register("summer@smith.family", "what.the.hell".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut original_order = domain::Order::sell_szabo(user.owner_id.unwrap(), 31929, 250000, 10);
    let mut order = order_repo.create(&original_order).unwrap();

    order.status = domain::OrderStatus::Settling;
    order = order_repo.update(&order).unwrap();
    assert!(order_repo.delete(&order).is_err());
    //assert_eq!(domain::OrderStatus::Deleted, order.status);
}

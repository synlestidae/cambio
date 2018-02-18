use domain;
use repositories::{UserRepository, OrderRepository};
use repository::Repository;
use repository;
use tests::get_db_helper;

#[test]
fn test_creates_order() {
    let mut user = domain::User::new_register("rick@sanchez.space", "m000000rtyburp".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut order_repo = OrderRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    let mut order = domain::Order::buy_szabo(user.owner_id.unwrap(), 250000, 31929, 10);
    order = order_repo.create(&order).unwrap();
}

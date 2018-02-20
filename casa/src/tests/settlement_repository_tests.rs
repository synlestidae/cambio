use domain;
use repositories;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;

#[test]
fn settles_two_orders() {
    // create the user first
    let mut user1 = domain::User::new_register("burns@springfield.com", "excellent123".to_owned());
    let mut user2 = domain::User::new_register("homer@springfield.com", "excellent123".to_owned());
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    user1 = user_repo.create(&user1).unwrap(); 
    user2 = user_repo.create(&user2).unwrap(); 


    let mut payment_repo = repositories::UserPaymentRepository::new(get_db_helper());
    let payment_builder = domain::PaymentBuilder::new(domain::AssetType::NZD,
        domain::Denom::Cent,
        domain::PaymentMethod::NZBankDeposit,
        domain::PaymentVendor::Poli);
    let payment = payment_builder.transaction_details(
        "6bd9ace6-e2c9-4e7a-bad8-49dccb3701b9",
        Utc::now(),
        200 * 100).unwrap();

    let user_payment = domain::UserPayment {
        payment: payment,
        email_address: "burns@springfield.com".to_owned()
    };

    payment_repo.create(&user_payment).unwrap();

    // burns and homer now make orders
    let mut order1 = domain::Order::buy_szabo(user1.id.unwrap(), 10000, 200 * 100, 10);
    let mut order2 = domain::Order::sell_szabo(user1.id.unwrap(), 200 * 100, 10000, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    // the system matches the orders with a settlement
    let mut settlement = domain::OrderSettlement {
        id: None,
        started_at: Utc::now(),
        settled_at: None,
        starting_user: user2.id.unwrap(),
        settlement_status: domain::SettlementStatus::Settling,
        buying_order: order1.clone(), 
        selling_order: order2.clone()
    };
    settlement = settlement_repo.create(&settlement).unwrap();
}

use domain;
use repositories;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;
use services;
use uuid;

pub fn get_user(email: &str) -> domain::User {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    user_repo.read(&repository::UserClause::EmailAddress(email.to_owned())).unwrap().pop().unwrap()
}

pub fn quick_order(buyer: &str, seller: &str, buy_szabo: u64, sell_money: u32, sell_szabo: u64, buy_money: u32) -> (domain::Order, domain::Order) {
    // create the user first
    let mut user_service = services::UserService::new(get_db_helper(), "/Users/mate/work/cambio/eth_test/data/geth.ipc");

    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());

    let user1 = user_service.register_user(buyer, "excellent123".to_owned()).unwrap(); 
    let user2 = user_service.register_user(seller, "dohnut123".to_owned()).unwrap(); 

    let mut order1 = domain::Order::buy_szabo(user1.id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user2.id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    (order1, order2)
}


pub fn just_order(buyer: &str, seller: &str, buy_szabo: u64, sell_money: u32, sell_szabo: u64, buy_money: u32) -> (domain::Order, domain::Order) {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());

    let user1 = user_repo.read(&repository::UserClause::EmailAddress(buyer.to_owned()))
        .unwrap()
        .pop()
        .unwrap(); 
    let user2 = user_repo.read(&repository::UserClause::EmailAddress(seller.to_owned())) 
        .unwrap()
        .pop()
        .unwrap(); 

    let mut order1 = domain::Order::buy_szabo(user1.id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user1.id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    (order1, order2)
}

pub fn quick_credit(who: &str, how_much: u32) {
    let mut payment_repo = repositories::UserPaymentRepository::new(get_db_helper());
    let payment_builder = domain::PaymentBuilder::new(domain::AssetType::NZD,
        domain::Denom::Cent,
        domain::PaymentMethod::NZBankDeposit,
        domain::PaymentVendor::Poli);
    let payment = payment_builder.transaction_details(
        &uuid::Uuid::new_v4().to_string(),
        Utc::now(),
        how_much as i64).unwrap();

    let user_payment = domain::UserPayment {
        payment: payment,
        email_address: who.to_owned()
    };

    let payment = payment_repo.create(&user_payment).unwrap();
}


pub fn quick_credit_szabo(who: &str, how_much: u64) {
    unimplemented!()
}

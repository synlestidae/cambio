use chrono::prelude::*;
use db;
use domain;
use repositories;
use repository;
use repository::*;
use services;
use std::io::Read;
use tests::get_db_helper;
use uuid;
use web3;

pub fn get_user(email: &str) -> domain::User {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    user_repo
        .read(&repository::UserClause::EmailAddress(email.to_owned()))
        .unwrap()
        .pop()
        .unwrap()
}

pub fn quick_order(
    buyer: &str,
    seller: &str,
    buy_szabo: u64,
    sell_money: u32,
    sell_szabo: u64,
    buy_money: u32,
) -> (domain::Order, domain::Order) {
    // create the user first
    let mut user_service = services::UserService::new(
        get_db_helper(),
        "http://localhost:8081",
    );

    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());

    let user1 = user_service
        .register_user(buyer, "excellent123".to_owned())
        .unwrap();

    let user2 = user_service
        .register_user(seller, "dohnut123".to_owned())
        .unwrap();

    let mut order1 = domain::Order::buy_szabo(user1.owner_id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user2.owner_id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    (order1, order2)
}

pub fn just_order(
    buyer: &str,
    seller: &str,
    buy_szabo: u64,
    sell_money: u32,
    sell_szabo: u64,
    buy_money: u32,
) -> (domain::Order, domain::Order) {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());

    let user1 = user_repo
        .read(&repository::UserClause::EmailAddress(buyer.to_owned()))
        .unwrap()
        .pop()
        .unwrap();
    let user2 = user_repo
        .read(&repository::UserClause::EmailAddress(seller.to_owned()))
        .unwrap()
        .pop()
        .unwrap();

    let mut order1 = domain::Order::buy_szabo(user1.owner_id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user1.owner_id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    (order1, order2)
}

pub fn quick_credit(who: &str, how_much: u32) {
    let mut payment_repo = repositories::UserPaymentRepository::new(get_db_helper());
    let payment_builder = domain::PaymentBuilder::new(
        domain::AssetType::NZD,
        domain::PaymentMethod::NZBankDeposit,
        domain::PaymentVendor::Poli,
    );
    let payment = payment_builder
        .transaction_details(
            &uuid::Uuid::new_v4().to_string(),
            Utc::now(),
            how_much as i64,
        )
        .unwrap();

    let user_payment = domain::UserPayment {
        payment: payment,
        email_address: who.to_owned(),
    };

    let payment = payment_repo.create(&user_payment).unwrap();
}

pub fn quick_credit_szabo(who: &str, how_much: u64) {
    use std::process::Command;

    let mut wei = web3::types::U256::from(how_much);
    wei = wei.full_mul(web3::types::U256::exp10(12)).into();
    let mut eth_account_repo = repositories::EthAccountRepository::new(get_db_helper());
    let clause = repository::UserClause::EmailAddress(who.to_owned());
    let account = eth_account_repo.read(&clause).unwrap().pop().unwrap();
    let args = &[
        "../moneda/index.js",
        "http://localhost:8081",
        "0xA990F82d33Fd19C3872dc12c588A66224b9330A6",
        &format!("0x{:#x}", account.address),
        &format!("0x{:#x}", wei),
        "77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b",
    ];
    let mut output = Command::new("node")
        .args(args)
        .spawn()
        .expect("failed to execute process")
        .wait_with_output()
        .unwrap();
    if !output.status.success() {
        let error = String::from_utf8(output.stderr).unwrap();
        panic!("Failed to credit account. Program error.\n{}", error); //: '{}'\nError below: \n {}", stdout_str, stderr_str);// output: {}\n", err_str);
    }
}

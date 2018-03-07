use domain;
use std::io::Read;
use repositories;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;
use services;
use uuid;
use web3;

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
    use std::process::Command;

    let wei = web3::types::U256::from(how_much);
    let mut eth_account_repo = repositories::EthAccountRepository::new(get_db_helper());
    let clause = repository::UserClause::EmailAddress(who.to_owned());
    let account = eth_account_repo.read(&clause).unwrap().pop().unwrap();
    let args = &["../moneda/index.js", 
        "http://localhost:8080", 
        "0xA990F82d33Fd19C3872dc12c588A66224b9330A6",
        &format!("0x{:#x}", account.address), 
        &format!("0x{:#x}", wei),
        "77173c4b349c6342ae695f86c5610688606de77361769bd8919301fc55823f1b" 
    ];
    let output = Command::new("node")
        .args(args) 
        .spawn()
        .expect("failed to execute process");
        //.wait_with_output()
        //.expect("failed to get output");
    let mut buf = vec![0, 0, 0, 0, 0, 0, 0, 0];
    output.stdout.unwrap().read_exact(&mut buf);
    let stdout_str = String::from_utf8(buf).unwrap();
    //let output_str = String::from_utf8(output.stdout).unwrap();
    //let err_str = String::from_utf8(output.stderr).unwrap();
    if !stdout_str.starts_with("Success") {
        panic!("Failed to credit account. Program error.");// output: {}\n", err_str);
    }
}

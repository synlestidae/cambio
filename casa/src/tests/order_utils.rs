use api::PersonalDetails;
use chrono::prelude::*;
use db;
use domain;
use repository::{Creatable, Readable};
use services;
use std::io::Read;
use std::rc::Rc;
use tests::get_db_connection;
use uuid;
use web3;
use tests::test_utils::*;

pub fn get_user(email: &str) -> domain::User {
    Readable::get(email, &mut get_db_connection()).unwrap()
}

pub fn quick_order(
    buyer: &str,
    seller: &str,
    buy_szabo: u64,
    sell_money: u32,
    sell_szabo: u64,
    buy_money: u32,
) -> (domain::Order, domain::Order) {
    let mut db = get_db_connection();
    // create the user first
    use std::env;
    let path = env::current_dir().unwrap();
    let (eloop, web3) = get_web3();
    let mut user_service = services::UserService::new(web3);

    let user1 = user_service
        .register_user(&mut db, buyer, "excellent123", &fake_details())
        .unwrap();

    let user2 = user_service
        .register_user(&mut db, seller, "dohnut123", &fake_details())
        .unwrap();

    let mut order1 = domain::Order::buy_szabo(user1.owner_id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user2.owner_id.unwrap(), buy_money, sell_szabo, 10);

    let saved_order1 = order1.create(&mut db).unwrap();
    let saved_order2 = order2.create(&mut db).unwrap();

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
    let mut db = get_db_connection();
    let user1 = Readable::get(buyer, &mut db).unwrap();
    let user2 = Readable::get(seller, &mut db).unwrap();

    let mut order1 = domain::Order::buy_szabo(user1.owner_id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user1.owner_id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order1.create(&mut db).unwrap();
    order2 = order2.create(&mut db).unwrap();

    (order1, order2)
}

pub fn quick_credit(who: &str, how_much: u32) {
    let mut db = get_db_connection();
    let user: domain::User = Readable::get(who, &mut db).unwrap();
    let account_set =
        domain::AccountSet::from(user.owner_id.unwrap().get_vec(&mut db).unwrap()).unwrap();
    let ledger_service = services::LedgerService::new();
    let account_id = domain::PaymentVendor::Poli.get(&mut db).unwrap();
    ledger_service
        .transfer_money(
            &mut db,
            account_id,
            account_set.nzd_wallet(),
            domain::Decimal::from_cents(how_much as i64),
        )
        .unwrap();
}

pub fn quick_credit_szabo(who: &str, how_much: u64) {
    use std::process::Command;
    unimplemented!()
    /*let mut db = get_db_connection();
    let mut wei = web3::types::U256::from(how_much);
    wei = wei.full_mul(web3::types::U256::exp10(12)).into();
    let clause = repository::UserClause::EmailAddress(who.to_owned());
    let account: domain::EthAccount = unimplemented!();//eth_account_repo.read(&clause).unwrap().pop().unwrap();
    let args = &[
        "../moneda/index.js",
        IPC_ADDRESS,
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
    }*/
}

pub fn fake_details() -> PersonalDetails {
    PersonalDetails {
        first_names: "Jerry John".to_owned(),
        family_name: "Jackson".to_owned(),
        address_line_1: "43 Fake St".to_owned(),
        address_line_2: None,
        post_code: "1123".to_owned(),
        city: "Wellington".to_owned(),
        country: "NEW ZEALAND".to_owned(),
        dob: NaiveDate::from_ymd(1990, 11, 11),
        id_type: "NZ_Passport".to_owned(),
        id_number: "LM123309".to_owned(),
    }
}

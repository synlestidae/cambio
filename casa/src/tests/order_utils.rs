use api::PersonalDetails;
use chrono::prelude::*;
use db;
use domain::*;
use domain;
use repository::{Creatable, Readable};
use services;
use std::io::Read;
use std::rc::Rc;
use tests::get_db_connection;
use tests::test_utils::*;
use event::Bus;
use colectivo::Colectivo;
use uuid;
use web3;

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
    let bus = Bus::from_topic("orders");
    let colectivo = Colectivo::new();
    let mut user_service = services::UserService::new(Bus::from_topic("orders"));

    let user1 = user_service
        .register_user(&mut db, buyer, "excellent123", &fake_details())
        .unwrap();

    let user2 = user_service
        .register_user(&mut db, seller, "dohnut123", &fake_details())
        .unwrap();

    let mut order1 = make_buy(user1.owner_id.unwrap(), buy_szabo, sell_money);
    let mut order2 = make_sell(user2.owner_id.unwrap(), sell_szabo, buy_money);

    (
        order1.create(&mut db).unwrap(),
        order2.create(&mut db).unwrap(),
    )
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

    let mut order1 = make_buy(user1.owner_id.unwrap(), buy_szabo, sell_money);
    let mut order2 = make_sell(user1.owner_id.unwrap(), sell_szabo, buy_money);

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
            domain::AssetType::NZD,
            domain::Decimal::from_cents(how_much as i64),
        )
        .unwrap();
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
        //id_type: "NZ_Passport".to_owned(),
        //id_number: "LM123309".to_owned(),
    }
}

fn make_buy(owner_id: OwnerId, szabo: u64, dollars: u32) -> Order {
    unimplemented!()
}

fn make_sell(owner_id: OwnerId, szabo: u64, dollars: u32) -> Order {
    unimplemented!()
}

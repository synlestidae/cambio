use domain;
use repositories;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;
use services;
use uuid;

#[test]
fn settles_two_orders() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let (order1, order2) = quick_order("burns@springfield.com", "homer@simpson.com", 100000, 200*100, 100000, 200 * 100);
    quick_credit("burns@springfield.com", 200 * 100);
    let burns = get_user("burns@springfield.com");

    let mut settlement = domain::OrderSettlement::from(burns.id.unwrap(), &order1, &order2);
    let mut created_settlement = settlement_repo.create(&settlement).unwrap();
    created_settlement.id = None;
    assert_eq!(settlement.settlement_status, domain::SettlementStatus::Settling);
    assert_eq!(settlement.buying_order, created_settlement.buying_order);
    assert_eq!(settlement.selling_order, created_settlement.selling_order);
}


#[test]
fn refuses_unfair_order() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    // order 2 will pay one dollar less than what order1 wants
    let (order1, order2) = quick_order("marge@simpson.com", "lionel.hutz@springfield.com", 100000, 200*100, 100000, 199 * 100);
    quick_credit("marge@simpson.com", 200 * 100);
    let hutz = get_user("lionel.hutz@springfield.com");

    // the system matches the orders with a settlement
    let settlement = domain::OrderSettlement::from(hutz.id.unwrap(), &order1, &order2);
    let settlement_error = settlement_repo.create(&settlement);
    assert!(settlement_error.is_err());
}


#[test]
fn settlement_makes_money_unavailable() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let mut account_service = services::AccountService::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());

    let (order1, order2) = quick_order("seymour@skinner.com", "edna@krandel.com", 
        100000, 200 * 100, 100000, 200 * 100);
    let skinner = get_user("seymour@skinner.com");
    quick_credit("seymour@skinner.com", 200 * 100);

    // the system matches the orders with a settlement
    let mut settlement = domain::OrderSettlement::from(skinner.id.unwrap(), &order1, &order2) ;
    settlement_repo.create(&settlement).unwrap();
    let query = repository::UserClause::EmailAddress("seymour@skinner.com".to_owned());
    let account_list = account_repo.read(&query).unwrap();
    let accounts = domain::AccountSet::from(account_list).unwrap();
    let skinner_account = account_service.get_latest_statement(accounts.nzd_wallet()).unwrap();
    assert_eq!(0, skinner_account.closing_balance); 
}

#[test]
fn doesnt_settle_already_settle() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let mut account_service = services::AccountService::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());

    let (order1, order2) = quick_order("bort@simpson.com", "lisa@simpson.com", 
        100000, 200 * 100, 100000, 200 * 100);
    let (order3, order4) = quick_order("itchy@springfield.com", "scratchy@springfield.com", 
        100000, 200 * 100, 100000, 200 * 100);

    quick_credit("bort@simpson.com", 2 * 200 * 100);
    quick_credit("itchy@springfield.com", 2 * 200 * 100);

    let bort = get_user("bort@simpson.com");
    let itchy = get_user("itchy@springfield.com");
    let settlement1 = domain::OrderSettlement::from(bort.id.unwrap(), &order1, &order2);
    let settlement2 = domain::OrderSettlement::from(bort.id.unwrap(), &order3, &order4);
    let settlement3 = domain::OrderSettlement::from(itchy.id.unwrap(), &order1, &order4);
    let settlement4 = domain::OrderSettlement::from(itchy.id.unwrap(), &order2, &order3);
    settlement_repo.create(&settlement1).unwrap();
    settlement_repo.create(&settlement2).unwrap();
    assert!(settlement_repo.create(&settlement3).is_err());
    assert!(settlement_repo.create(&settlement4).is_err());
}


#[test]
fn doesnt_settle_when_credit_runs_out() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let mut account_service = services::AccountService::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    const MONEY: u32 = 200 * 100;

    let (order1, order2) = quick_order("ralph@wiggum.com", "chief@wiggum.com", 
        100000, MONEY, 100000, MONEY);

    let (order3, order4) = just_order("ralph@wiggum.com", "chief@wiggum.com", 
        100000, MONEY, 100000, MONEY);

    let ralph = get_user("ralph@wiggum.com");
    quick_credit("ralph@wiggum.com",  MONEY);
    let settlement1 = domain::OrderSettlement::from(ralph.id.unwrap(), &order1, &order2);
    let settlement2 = domain::OrderSettlement::from(ralph.id.unwrap(), &order3, &order4);
    assert!(settlement_repo.create(&settlement1).is_ok());
    assert!(settlement_repo.create(&settlement2).is_err());
}

pub fn get_user(email: &str) -> domain::User {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    user_repo.read(&repository::UserClause::EmailAddress(email.to_owned())).unwrap().pop().unwrap()
}

pub fn quick_order(buyer: &str, seller: &str, buy_szabo: u64, sell_money: u32, sell_szabo: u64, buy_money: u32) -> (domain::Order, domain::Order) {
    // create the user first
    let mut user1 = domain::User::new_register(buyer, "excellent123".to_owned());
    let mut user2 = domain::User::new_register(seller, "dohnut123".to_owned());
    let mut user_repo = repositories::UserRepository::new(get_db_helper());

    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());

    user1 = user_repo.create(&user1).unwrap(); 
    user2 = user_repo.create(&user2).unwrap(); 

    let mut order1 = domain::Order::buy_szabo(user1.id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user1.id.unwrap(), buy_money, sell_szabo, 10);

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

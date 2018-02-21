use domain;
use repositories;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;
use uuid;

#[test]
fn settles_two_orders() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let (order1, order2) = quick_order("burns@springfield.com", "homer@simpson.com", 100000, 200*100, 100000, 200 * 100);
    let burns = get_user("burns@springfield.com");

    // the system matches the orders with a settlement
    let mut settlement = domain::OrderSettlement {
        id: None,
        started_at: Utc::now(),
        settled_at: None,
        starting_user: burns.id.unwrap(),
        settlement_status: domain::SettlementStatus::Settling,
        buying_order: order1, 
        selling_order: order2
    };

    settlement = settlement_repo.create(&settlement).unwrap();
    assert_eq!(settlement.settlement_status, domain::SettlementStatus::Settling);
}


#[test]
fn refuses_unfair_order() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    // order 2 will pay one dollar less than what order1 wants
    let (order1, order2) = quick_order("marge@simpson.com", "lionel.hutz@springfield.com", 100000, 200*100, 100000, 199 * 100);
    let hutz = get_user("lionel.hutz@springfield.com");

    // the system matches the orders with a settlement
    let mut settlement = domain::OrderSettlement {
        id: None,
        started_at: Utc::now(),
        settled_at: None,
        starting_user: hutz.id.unwrap(),
        settlement_status: domain::SettlementStatus::Settling,
        buying_order: order1, 
        selling_order: order2
    };

    let settlement_error = settlement_repo.create(&settlement);
    assert!(settlement_error.is_err());
}


#[test]
fn settlement_makes_money_unavailable() {
    let mut settlement_repo = repositories::SettlementRepository::new(get_db_helper());
    let mut account_service = db::AccountService::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());

    let (order1, order2) = quick_order("seymour@skinner.com", "edna@krandel.com", 
        100000, 200*100, 100000, 200 * 100);
    let skinner = get_user("seymour@skinner.com");

    // the system matches the orders with a settlement
    let mut settlement = domain::OrderSettlement {
        id: None,
        started_at: Utc::now(),
        settled_at: None,
        starting_user: skinner.id.unwrap(),
        settlement_status: domain::SettlementStatus::Settling,
        buying_order: order1, 
        selling_order: order2
    };
    settlement_repo.create(&settlement).unwrap();
    let query = repository::UserClause::EmailAddress("seymour@skinner.com".to_owned());
    let account_list = account_repo.read(&query).unwrap();
    let accounts = domain::AccountSet::from(account_list).unwrap();
    let skinner_account = account_service.get_latest_statement(accounts.nzd_wallet()).unwrap();
    assert_eq!(0, skinner_account.closing_balance); 
}

fn get_user(email: &str) -> domain::User {
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    user_repo.read(&repository::UserClause::EmailAddress(email.to_owned())).unwrap().pop().unwrap()
}

fn quick_order(buyer: &str, seller: &str, buy_szabo: u64, sell_money: u32, sell_szabo: u64, buy_money: u32) -> (domain::Order, domain::Order) {
    // create the user first
    let mut user1 = domain::User::new_register(buyer, "excellent123".to_owned());
    let mut user2 = domain::User::new_register(seller, "dohnut123".to_owned());
    let mut user_repo = repositories::UserRepository::new(get_db_helper());
    let mut account_repo = repositories::AccountRepository::new(get_db_helper());
    let mut order_repo = repositories::OrderRepository::new(get_db_helper());
    user1 = user_repo.create(&user1).unwrap(); 
    user2 = user_repo.create(&user2).unwrap(); 


    let mut payment_repo = repositories::UserPaymentRepository::new(get_db_helper());
    let payment_builder = domain::PaymentBuilder::new(domain::AssetType::NZD,
        domain::Denom::Cent,
        domain::PaymentMethod::NZBankDeposit,
        domain::PaymentVendor::Poli);
    let payment = payment_builder.transaction_details(
        &uuid::Uuid::new_v4().to_string(),
        Utc::now(),
        200 * 100).unwrap();

    let user_payment = domain::UserPayment {
        payment: payment,
        email_address: buyer.to_owned()
    };

    payment_repo.create(&user_payment).unwrap();

    // burns and homer now make orders
    let mut order1 = domain::Order::buy_szabo(user1.id.unwrap(), buy_szabo, sell_money, 10);
    let mut order2 = domain::Order::sell_szabo(user1.id.unwrap(), buy_money, sell_szabo, 10);

    order1 = order_repo.create(&order1).unwrap();
    order2 = order_repo.create(&order2).unwrap();

    (order1, order2)
}

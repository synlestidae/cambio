use domain;
use repositories::AccountRepository;
use repositories::UserRepository;
use repositories::UserPaymentRepository;
use repository::*;
use repository;
use tests::get_db_helper;
use chrono::prelude::*;
use db;

#[test]
fn credits_account() {
    // create the user first
    let user = domain::User::new_register("anne@economist.com", "iamagoodjournalist123".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    user_repo.create(&user).unwrap(); 

    let mut payment_repo = UserPaymentRepository::new(get_db_helper());
    let payment_builder = domain::PaymentBuilder::new(domain::AssetType::NZD,
        domain::Denom::Cent,
        domain::PaymentMethod::NZBankDeposit,
        domain::PaymentVendor::Poli);
    let payment = payment_builder.transaction_details(
        "6bd9ace6-e2c9-4e7a-bad8-49dccb3701b8",
        Utc::now(),
        200 * 100).unwrap();

    let user_payment = domain::UserPayment {
        payment: payment,
        email_address: "anne@economist.com".to_owned()
    };

    payment_repo.create(&user_payment).unwrap();
    let accounts = account_repo.read(&repository::UserClause::EmailAddress("anne@economist.com".to_owned())).unwrap();
    let set = domain::AccountSet::from(accounts).unwrap();
    let mut account_service = db::AccountService::new(get_db_helper());
    let statement = account_service.get_latest_statement(set.nzd_wallet()).unwrap();
    assert_eq!(200 * 100, statement.closing_balance);
}

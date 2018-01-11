use db::{PostgresHelperImpl, UserRepository, AccountRepository, PaymentRepository};
use domain::{Payment, AssetType, Denom, PaymentVendor, PaymentMethod};
use chrono::prelude::*;
use std::process;
use tests::test_utils::*;

#[test]
fn user_account_gets_credited() {
    run_test(|| {
        let mut account_repository = AccountRepository::new(get_db_helper());
        let mut user_repository = UserRepository::new(get_db_helper());
        let mut payment_repository = get_repository();

        let username = "mate@cambio.co.nz";
        let password = "super_secRet_password_123";
        let credit = (50 * 100) + 50;

        user_repository.register_user(username, password.to_owned());

        //let payment = Payment
        let payment = Payment {
            unique_id: "tx_00000000000001".to_owned(),
            asset_type: AssetType::NZD,
            asset_denom: Denom::Cent,
            datetime_payment_made: Utc::now(),
            payment_method: PaymentMethod::CreditCard,
            vendor: PaymentVendor::Poli,
            user_credit: credit, // $50.00,
            message: Some("Test credit card payment into test account".to_owned()),
        };

        payment_repository.register_credit_payment("mate@cambio.co.nz", &payment);

        println!("Getting accounts");

        let accounts = account_repository
            .get_accounts_for_user("mate@cambio.co.nz")
            .unwrap();

        println!("Looking for the right one");

        let account = accounts
            .into_iter()
            .filter(|a| {
                a.asset_type == AssetType::NZD && a.asset_denom == Denom::Cent
            })
            .collect::<Vec<_>>()
            .pop()
            .unwrap();

        println!("Getting statement");

        let statement = account_repository
            .get_latest_statement(&account.id.unwrap())
            .unwrap();

        assert_eq!(credit, statement.closing_balance);
    });
}

fn get_repository() -> PaymentRepository<PostgresHelperImpl> {
    let account_repository = AccountRepository::new(get_db_helper());
    let user_repository = UserRepository::new(get_db_helper());
    PaymentRepository::new(get_db_helper(), account_repository, user_repository)
}

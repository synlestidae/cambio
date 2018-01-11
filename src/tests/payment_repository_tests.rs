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

        let statement = payment_repository.register_credit_payment("mate@cambio.co.nz",
                                                                   &payment).unwrap();

        println!("Getting statement");

        assert_eq!(credit, statement.closing_balance);

        let next_payment = Payment {
            unique_id: "tx_00000000000002".to_owned(),
            asset_type: AssetType::NZD,
            asset_denom: Denom::Cent,
            datetime_payment_made: Utc::now(),
            payment_method: PaymentMethod::CreditCard,
            vendor: PaymentVendor::Poli,
            user_credit: 30 * 100,
            message: Some("Test credit card payment into test account".to_owned()),
        };

        let next_statement = payment_repository.register_credit_payment("mate@cambio.co.nz", &next_payment)
            .unwrap();

        println!("next one: {:?}", next_statement);

        assert_eq!(3000 + (50 * 100) + 50, next_statement.closing_balance);
    });
}

fn get_repository() -> PaymentRepository<PostgresHelperImpl> {
    let account_repository = AccountRepository::new(get_db_helper());
    let user_repository = UserRepository::new(get_db_helper());
    PaymentRepository::new(get_db_helper(), account_repository, user_repository)
}

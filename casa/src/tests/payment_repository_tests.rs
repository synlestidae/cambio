use db::{PostgresHelperImpl, UserService, AccountRepository, PaymentRepository};
use domain::{Payment, AssetType, Denom, PaymentVendor, PaymentMethod, PaymentBuilder};
use chrono::prelude::*;
use std::process;
use tests::test_utils::*;

#[test]
fn user_account_gets_credited() {
    let mut account_repository = AccountRepository::new(get_db_helper());
    let mut user_service = UserService::new(get_db_helper());
    let mut payment_repository = get_repository();

    let username = "freddy@cambio.co.nz";
    let password = "super_secRet_password_123";
    user_service.register_user(username, password.to_owned());

    let credit_1 = (50 * 100) + 50;
    let credit_2 = (3000 * 100) + 0;

    let mut payment =
        PaymentBuilder::new(
            AssetType::NZD,
            Denom::Cent,
            PaymentMethod::CreditCard,
            PaymentVendor::Poli,
        ).transaction_details("14e3a50d-84e0-45d8-8981-6231cc8425bb", Utc::now(), credit_1)
            .unwrap();
    let statement = payment_repository
        .register_credit_payment("freddy@cambio.co.nz", &payment)
        .unwrap();

    assert_eq!(credit_1, statement.closing_balance);
    assert_eq!(1, statement.transactions.len());

    let mut next_payment =
        PaymentBuilder::new(
            AssetType::NZD,
            Denom::Cent,
            PaymentMethod::CreditCard,
            PaymentVendor::Poli,
        ).transaction_details("2ab8a43f-eed7-4f66-bcf8-8d3aa3490f9b", Utc::now(), credit_2)
            .unwrap();

    let next_statement = payment_repository
        .register_credit_payment("freddy@cambio.co.nz", &next_payment)
        .unwrap();
    let failed_payment =
        payment_repository.register_credit_payment("freddy@cambio.co.nz", &next_payment);

    assert_eq!(credit_1 + credit_2, next_statement.closing_balance);
    assert_eq!(2, next_statement.transactions.len());
    assert!(failed_payment.is_err());
}

#[allow(dead_code)]
fn get_repository() -> PaymentRepository<PostgresHelperImpl> {
    let account_repository = AccountRepository::new(get_db_helper());
    let user_service = UserService::new(get_db_helper());
    PaymentRepository::new(get_db_helper(), account_repository, user_service)
}

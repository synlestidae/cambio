use domain;
use repositories::{UserRepository, OrderRepository, AccountRepository};
use repository::*;
use repository;
use tests::get_db_helper;

#[test]
fn test_payment_saved() {
    let payment = domain::PaymentBuilder::new(domain::AssetType::NZD, 
        domain::AssetType::NZD,
        domain::Denom::Cents,
        domain::PaymentVendor::Poli,
        domain::PaymentMethod::NZBankDeposit);

    let payment = payment.transaction_details(
        "c0f56218-14df-11e8-b642-0ed5f89f718b", 
        Utc::now(),
        200 * 100);

    let mut user = domain::User::new_register("bill@microsoft.com", "$$$malariasucks".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut payment_repo = PaymentRepository::new(get_db_helper());
    user = user_repo.create(&user).unwrap(); 

    payment_repo.create(&payment).unwrap();

}

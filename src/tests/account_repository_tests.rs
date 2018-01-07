use db::{PostgresHelper, PostgresHelperImpl, UserRepository, AccountRepository};
use domain::{AssetType, Denom, AccountStatus, AccountBusinessType, AccountRole};
use chrono::prelude::*;
use std::process;
use std;
use tests::test_utils::*;

#[test]
fn test_gets_main_wallet_account_for_registered_user() {
    run_test(|| {
        println!("Running the dang test!");
        let mut account_repository = AccountRepository::new(get_db_helper());
        let mut user_repository = UserRepository::new(get_db_helper());
        let email_address = "mate@cambio.co.nz";
        let password = "$23@@super_secret_password";

        println!("Registering the boi");
        let user = user_repository.register_user(email_address, password.to_owned()).unwrap().unwrap();
        println!("Getting accounts for user");
        let accounts = account_repository.get_accounts_for_user(email_address).unwrap();
        let account = &accounts[0];

        assert!(false);

        assert_eq!(user.id.unwrap(), account.owner_user_id.unwrap());
        assert_eq!(AssetType::NZD, account.asset_type);
        assert_eq!(Denom::Cent, account.asset_denom);
        assert_eq!(AccountStatus::Active, account.account_status);
        assert_eq!(AccountBusinessType::UserCashWallet, account.account_business_type);
        assert_eq!(AccountRole::Primary, account.account_role);
    });
}

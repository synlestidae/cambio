use domain;
use repositories::AccountRepository;
use repositories::UserRepository;
use repository::Repository;
use repository;
use tests::get_db_helper;


#[test]
fn creates_all_accounts_for_user() {
    // create the user first
    let user = domain::User::new_register("jerry@waller.fm", "jerryjacksonfilms".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    user_repo.create(&user).unwrap(); 

    // get the account collection
    let accounts = account_repo.read(&repository::UserClause::EmailAddress("jerry@waller.fm".to_owned())).unwrap();
    let accounts = domain::AccountSet::from(accounts).unwrap();
    let wallet = account_repo.read(&repository::UserClause::Id(accounts.nzd_wallet()))
        .unwrap()
        .pop()
        .unwrap();

    let wallet = account_repo.read(&repository::UserClause::Id(accounts.nzd_wallet()))
        .unwrap()
        .pop()
        .unwrap();

    let hold = account_repo.read(&repository::UserClause::Id(accounts.nzd_hold()))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(wallet.asset_type, domain::AssetType::NZD);
    assert_eq!(wallet.asset_denom, domain::Denom::Cent);
    assert_eq!(wallet.account_type, domain::AccountType::Liability);
    assert_eq!(wallet.account_business_type, domain::AccountBusinessType::UserCashWallet);

    assert_eq!(hold.asset_type, domain::AssetType::NZD);
    assert_eq!(hold.asset_denom, domain::Denom::Cent);
    assert_eq!(hold.account_type, domain::AccountType::Liability);
    assert_eq!(hold.account_business_type, domain::AccountBusinessType::OrderPaymentHold);
}

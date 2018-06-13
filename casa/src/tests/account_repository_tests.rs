use domain;
use repositories::AccountRepository;
use repositories::UserRepository;
use repository;
use repository::*;
use tests::get_db_helper;

#[test]
fn creates_all_accounts_for_user() {
    // create the user first
    let user = domain::User::new_register("jerry@waller.fm", "jerryjacksonfilms".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    user_repo.create(&user).unwrap();

    // get the account collection
    let accounts = account_repo
        .read(&repository::UserClause::EmailAddress(
            "jerry@waller.fm".to_owned(),
        ))
        .unwrap();
    let accounts = domain::AccountSet::from(accounts).unwrap();
    let wallet = account_repo
        .read(&repository::UserClause::Id(accounts.nzd_wallet()))
        .unwrap()
        .pop()
        .unwrap();

    let wallet = account_repo
        .read(&repository::UserClause::Id(accounts.nzd_wallet()))
        .unwrap()
        .pop()
        .unwrap();

    let hold = account_repo
        .read(&repository::UserClause::Id(accounts.nzd_hold()))
        .unwrap()
        .pop()
        .unwrap();

    assert_eq!(wallet.asset_type, domain::AssetType::NZD);
    assert_eq!(wallet.account_type, domain::AccountType::Liability);
    assert_eq!(
        wallet.account_business_type,
        domain::AccountBusinessType::UserCashWallet
    );

    assert_eq!(hold.asset_type, domain::AssetType::NZD);
    assert_eq!(hold.account_type, domain::AccountType::Liability);
    assert_eq!(
        hold.account_business_type,
        domain::AccountBusinessType::OrderPaymentHold
    );
}

#[test]
fn no_accounts_created_for_empty_user() {
    // create the user first
    let user = domain::User::new_register("", "nothing".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    let user_result = user_repo.create(&user);
    assert!(user_result.is_err());

    // get the account collection
    let accounts = account_repo.read(&repository::UserClause::EmailAddress("".to_owned()));
    assert!(accounts.is_err());
}

#[test]
fn accounts_dont_exist() {
    let mut account_repo = AccountRepository::new(get_db_helper());
    // get the account collection
    let accounts = account_repo.read(&repository::UserClause::EmailAddress(
        "nobody@waller.fm".to_owned(),
    ));
    assert!(accounts.is_err());
}

#[test]
fn fails_create_duplicate_account() {
    // create the user first
    let user = domain::User::new_register("graham@waller.fm", "adventurequestiongame".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    user_repo.create(&user).unwrap();

    let account = domain::Account {
        id: None,
        owner_user_id: user.owner_id,
        asset_type: domain::AssetType::NZD,
        account_status: domain::AccountStatus::Active,
        account_business_type: domain::AccountBusinessType::UserCashWallet,
        account_type: domain::AccountType::Liability,
        account_role: domain::AccountRole::Primary,
    };

    assert!(account_repo.create(&account).is_err());
}

#[test]
fn freezes_accounts() {
    const SANDY: &'static str = "sandy.scawthorpe@waller.fm";
    // create the user first
    let user = domain::User::new_register(SANDY, "nolove".to_owned());
    let mut user_repo = UserRepository::new(get_db_helper());
    let mut account_repo = AccountRepository::new(get_db_helper());
    user_repo.create(&user).unwrap();

    let accounts = account_repo
        .read(&repository::UserClause::EmailAddress(SANDY.to_owned()))
        .unwrap();
    let account_set = domain::AccountSet::from(accounts).unwrap();

    let wallet_clause = repository::UserClause::Id(account_set.nzd_wallet());
    let mut wallet = account_repo.read(&wallet_clause).unwrap().pop().unwrap();

    wallet.account_status = domain::AccountStatus::Frozen;
    account_repo.update(&wallet).unwrap();

    wallet = account_repo.read(&wallet_clause).unwrap().pop().unwrap();

    assert_eq!(wallet.account_status, domain::AccountStatus::Frozen);
}

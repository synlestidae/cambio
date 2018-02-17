use domain;
use repositories::UserRepository;
use repository::Repository;
use repository;
use tests::get_db_helper;

#[test]
fn creates_one_user() {
    let mut repo = UserRepository::new(get_db_helper());
    let user = domain::User::new_register("jhon@fernando.com", 
        "ilovecali123".to_owned());
    let created_user = (repo.create(&user)).unwrap();
    let email_clause = repository::UserClause::EmailAddress("jhon@fernando.com".to_owned());
    let mut users = repo.read(&email_clause).unwrap();
    let found_user = users.pop().unwrap();

    assert_eq!(created_user, found_user);

    let id_clause = repository::UserClause::Id(found_user.id.unwrap());
    users = repo.read(&id_clause).unwrap();
    let id_user = users.pop().unwrap();

    assert_eq!(created_user, id_user);

    let mut all_users = repo.read(&repository::UserClause::All(true)).unwrap();
    let mut our_guy = all_users.into_iter()
        .filter(|ref u| u.id == found_user.id)
        .collect::<Vec<_>>()
        .pop()
        .unwrap();

    assert_eq!(created_user, our_guy);

    assert!(created_user.hash_matches_password("ilovecali123"));
    assert!(!created_user.hash_matches_password("il0vebogota"));
    assert!(!created_user.hash_matches_password("i"));
    assert!(!created_user.hash_matches_password("jerry_jackson"));

    our_guy.change_password("il0vebogota");

    let updated_guy = repo.update(&our_guy).unwrap();
    updated_guy.hash_matches_password("il0vebogota");
}

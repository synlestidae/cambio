use db::{PostgresHelper, PostgresHelperImpl, UserRepository};
use postgres::{Connection, TlsMode};
use std::process::Command;
use std::env::current_dir;
use chrono::prelude::*;
use std::panic::catch_unwind;
use std::process;
use std;

#[test]
fn test_get_user_returns_none_for_nonexistent_user() {
    run_test(|| {
        let mut user_repository = get_repository();
        assert_eq!(Ok(None), user_repository.get_user_by_email("mate@cambio.co.nz"));
    });
}

#[test]
fn test_get_user_returns_none_for_empty_string() {
    run_test(|| {
        let mut user_repository = get_repository();
        assert_eq!(Ok(None), user_repository.get_user_by_email(""));
    });
}

#[test]
fn test_get_user_returns_none_for_malformed_email_address() {
    run_test(|| {
        let mut user_repository = get_repository();
        assert_eq!(Ok(None), user_repository.get_user_by_email("mate@@cambio.co.nz"));
    });
}

#[test]
fn test_get_user_returns_user_after_register() {
    run_test(|| {
        let mut user_repository = get_repository();
        user_repository.register_user("mate@cambio.co.nz", "$2youwillnevergUess".to_owned()).unwrap();
        assert_eq!(user_repository.get_user_by_email("mate@cambio.co.nz").unwrap().unwrap().email_address, "mate@cambio.co.nz");
    });
}

#[test]
fn test_register_user_rejects_malformed_email_addresses() {
    run_test(|| {
        let mut user_repository = get_repository();
        let password = "8923qjtrfqr7q23r_luNch";
        assert!(user_repository.register_user("mate@@cambio.co.nz", password.to_owned()).is_err());
        assert!(user_repository.register_user("@cambio.co.nz", password.to_owned()).is_err());
        assert!(user_repository.register_user("@", password.to_owned()).is_err());
        assert!(user_repository.register_user("@@", password.to_owned()).is_err());
        assert!(user_repository.register_user("mate@.cambio.co.nz", password.to_owned()).is_err());
    });
}

#[test]
fn test_register_user_allows_login_and_logout() {
    run_test(|| {
        let mut user_repository = get_repository();
        let password = "8923qjtrfqr7q23r_luNch";
        let email = "mate@cambio.co.nz";
        user_repository.register_user(email, password.to_owned());
        let session = user_repository.log_user_in(email, password.to_owned()).unwrap().unwrap();
        let other_session = user_repository.get_existing_session(email, &session.session_token).unwrap().unwrap();

        assert!(session.session_token.len() > 32);
        assert_eq!(session.email_address, email);
        assert!(session.expires_at > Utc::now());
        assert_eq!(session, other_session);

        user_repository.log_user_out(email);
        assert_eq!(user_repository.get_existing_session(email, &session.session_token), Ok(None));
    });
}

fn do_login_logout_test() {
}

fn setup() {
    let conn = Connection::connect("postgresql://mate@localhost:5432", TlsMode::None).unwrap();
    conn.execute("CREATE DATABASE test_database_only;", &[]);
    let output = Command::new("bash")
        .arg("src/tests/setup_db.sh")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap()
        .wait().unwrap();
}

fn teardown() {
    let conn = Connection::connect("postgresql://mate@localhost:5432", TlsMode::None).unwrap();
    conn.execute("DROP DATABASE test_database_only;", &[]);
}

fn run_test<T: std::panic::UnwindSafe>(test: T) -> ()
    where T: FnOnce() -> ()
{
    setup();

    catch_unwind(|| {
        test()
    }).unwrap();

    teardown();
}

fn get_repository() -> UserRepository<PostgresHelperImpl> {
    let conn = Connection::connect("postgres://mate@localhost:5432/test_database_only", TlsMode::None)
        .unwrap();
    let db = PostgresHelperImpl::new(conn);
    UserRepository::new(db)

}

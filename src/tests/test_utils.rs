use db::{PostgresHelperImpl, PostgresSource};
use postgres::{Connection, TlsMode};
use std::process::Command;
use std::panic::catch_unwind;
use std;

#[allow(dead_code)]
pub fn setup() {
    let conn = Connection::connect("postgresql://mate@localhost:5432", TlsMode::None).unwrap();
    conn.execute("CREATE DATABASE test_database_only;", &[])
        .unwrap();
    let output = Command::new("bash")
        .arg("src/tests/setup_db.sh")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

#[allow(dead_code)]
pub fn teardown() {
    let conn = Connection::connect("postgresql://mate@localhost:5432", TlsMode::None).unwrap();
    match conn.execute("DROP DATABASE test_database_only;", &[]) {
        Ok(_) => {}
        Err(err) => println!("Failed to drop database: {}", err),
    }
}

#[allow(dead_code)]
pub fn run_test<T: std::panic::UnwindSafe>(test: T) -> ()
where
    T: FnOnce() -> (),
{
//    setup();

    test();
    //let result = catch_unwind(|| test());

//    teardown();

    //result.unwrap();
}

#[allow(dead_code)]
pub fn get_db_helper() -> PostgresHelperImpl {
    let source = PostgresSource::new("postgres://mate@localhost:5432/test_database_only").unwrap();
    PostgresHelperImpl::new(source)
}

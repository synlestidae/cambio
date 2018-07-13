use api;
use postgres;
use jobs::JobRequest;
use db::ConnectionSource;
use std::sync::mpsc::{Receiver, Sender};
use chrono::NaiveDate;
use iron::status::Status;
use bcrypt::hash;
use db::{PostgresHelperImpl, PostgresSource};
use iron::headers::Headers;
use iron_test::{request, response};
use postgres::{Connection, TlsMode};
use serde::{Serialize, Deserialize};
use serde_json;
use services::UserService;
use std::panic::catch_unwind;
use std::process::Command;
use std::sync::mpsc::channel;
use std;
use api::PersonalDetails;

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
    test();
}

pub const TEST_CONN_STR: &'static str = "postgres://mate@localhost:5432/test_database_only";

pub fn get_db_source() -> PostgresSource {
    PostgresSource::new(TEST_CONN_STR).unwrap()
}

pub fn get_db_connection() -> postgres::Connection {
    Connection::connect(TEST_CONN_STR, TlsMode::None).unwrap()
}

#[allow(dead_code)]
pub fn get_db_helper() -> PostgresHelperImpl {
    PostgresHelperImpl
}

pub fn log_in(username: &str, password: &str) -> String {
    let mut db = get_db_connection();
    let mut user_service = UserService::new("http://localhost:8081"); 
    let user = user_service.create_user(
        &mut db,
        username, 
        &hash(password, 6).unwrap(), 
        &PersonalDetails {
            first_names: "Jerry".to_string(),
            family_name: "Jackson".to_string(),
            address_line_1: "44 Jackson St".to_string(),
            address_line_2: None,
            post_code: "4231".to_string(),
            city: "Newville".to_string(),
            country: "NEW ZEALAND".to_string(),
            dob: NaiveDate::from_ymd(1990, 1, 1),
            id_type: "NZ_Passport".to_string(),
            id_number: "LM008381".to_string()
        },
        password
    );
    user_service.log_user_in(&mut db, username, password.to_owned()).unwrap().session_token.0
}

pub fn post_channel<'a, E: Serialize>(url: &str, token: Option<&str>, obj: Option<E>, tx: Sender<JobRequest>) -> String {
    make_request(url, token, obj, false, tx)
}

pub fn post<'a, E: Serialize>(url: &str, token: Option<&str>, obj: Option<E>) -> String {
    let (tx, rx) = channel();
    make_request(url, token, obj, false, tx)
}

pub fn get<'a, E: Serialize>(url: &str, token: Option<&str>) -> String {
    let (tx, rx) = channel();
    make_request(url, token, None as Option<()>, true, tx)
}

fn make_request<'a, E: Serialize>(url: &str, token: Option<&str>, obj: Option<E>, is_get: bool, tx: Sender<JobRequest>) -> String {
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    if let Some(t) = token {
        headers.set_raw("Authorization", vec![format!("Bearer {}", t).into_bytes()])
    }
    let handler = api::ApiHandler::new(TEST_CONN_STR, "http://localhost:8081", tx);
    let response = if is_get {
        request::get(url, 
            headers.clone(), 
            &handler).unwrap()
    } else {
        request::post(url, 
            headers.clone(), 
            &serde_json::to_string(&obj).unwrap(),
            &handler).unwrap()
    };

    let status = response.status.clone();
    let body = response::extract_body_to_string(response);

    if status != Some(Status::Ok) {
        panic!("Response for {} had status {:?}: {}", url, status, body);
    }

    body
}

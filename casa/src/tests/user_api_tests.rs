use api;
use iron::prelude::*;
use iron::{Handler, Headers};
use iron::status::Status;
use iron_test::{request, response};
use tests::test_utils::get_db_helper;
use serde_json;
use domain;

#[test]
fn test_creates_new_user() {
    let new_user = r#"{
        "email_address": "mate@coolcat.com",
        "password": "supersecret123"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let handler = api::ApiHandler::new(get_db_helper(), "http://localhost:8081");
    let response = request::post("http://localhost:3000/users/register", 
        headers, 
        new_user,
        &handler).unwrap();
    let result_body = response::extract_body_to_bytes(response);
    let user: domain::User = serde_json::from_slice(&result_body).unwrap();
    assert!(!user.id.is_none());
    assert_eq!("mate@coolcat.com", user.email_address);
    assert_eq!(None, user.password);
    assert_eq!(None, user.password_hash);
    assert_eq!(None, user.owner_id);
}

#[test]
fn test_creates_new_user_and_password_works() {
    let new_user = r#"{
        "email_address": "cat@coolcat.com",
        "password": "supersecret1234"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let handler = api::ApiHandler::new(get_db_helper(), "http://localhost:8081");
    request::post("http://localhost:3000/users/register", 
        headers.clone(), 
        new_user,
        &handler).unwrap();

    let response = request::post("http://localhost:3000/users/log_in", 
        headers, 
        r#"{
            "email_address": "cat@coolcat.com",
            "password": "supersecret123"
        }"#,
        &handler).unwrap();

    assert_eq!(Status::Unauthorized, response.status.unwrap());
    let result_body = response::extract_body_to_bytes(response);
    let result: api::ApiError= serde_json::from_slice(&result_body).unwrap();
}


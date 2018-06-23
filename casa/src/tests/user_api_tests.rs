use api;
use iron::prelude::*;
use iron::{Handler, Headers};
use iron::status::Status;
use iron_test::{request, response};
use tests::test_utils::get_db_helper;
use serde_json;
use repository::Readable;
use domain;
use std::sync::mpsc::channel;
use serde::Serialize;

#[test]
fn test_registration_successful() {
    let new_user = r#"{
        "email_address": "mate@coolcat.com",
        "password": "supersecret123"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let (tx, rx) = channel();
    let handler = api::ApiHandler::new(get_db_helper(), "http://localhost:8081", tx);
    let response = request::post("http://localhost:3000/users/register", 
        headers, 
        new_user,
        &handler).unwrap();
    let result_body = response::extract_body_to_string(response);
    let reg_result: api::RegistrationInfo = serde_json::from_str(&result_body).unwrap();
    assert_eq!("mate@coolcat.com", reg_result.email_address);
    assert_eq!(20, reg_result.identifier_code.0.len());
}

#[test]
fn test_creates_new_user_and_password_works() {
    let mut db = get_db_helper();
    let new_user = r#"{
        "email_address": "cat@coolcat.com",
        "password": "supersecret1234"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let (tx, rx) = channel();
    let handler = api::ApiHandler::new(db.clone(), "http://localhost:8081", tx);
    let register_response = request::post("http://localhost:3000/users/register", 
        headers.clone(), 
        new_user,
        &handler).unwrap();

    let result_body = response::extract_body_to_string(register_response);
    let reg_result: api::RegistrationInfo = serde_json::from_str(&result_body).unwrap();
    let registration: domain::Registration = reg_result.identifier_code.get(&mut db).unwrap();
    let confirmation = api::RegistrationConfirm {
        email_address: "cat@coolcat.com".to_owned(),
        confirmation_code: registration.confirmation_code,    
        identifier_code: registration.identifier_code
    };

    request::post("http://localhost:3000/users/confirm", 
        headers.clone(), 
        &serde_json::to_string(&confirmation).unwrap(),
        &handler).unwrap();

    let login_response = request::post("http://localhost:3000/users/log_in", 
        headers.clone(), 
        r#"{
            "email_address": "cat@coolcat.com",
            "password": "supersecret123"
        }"#,
        &handler).unwrap();

    assert_eq!(Status::Ok, login_response.status.unwrap());


}


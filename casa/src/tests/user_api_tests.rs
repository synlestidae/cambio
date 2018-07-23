use api;
use domain;
use iron::prelude::*;
use iron::status::Status;
use iron::{Handler, Headers};
use iron_test::{request, response};
use repository::Readable;
use serde::Serialize;
use serde_json;
use std::sync::mpsc::channel;
use tests::test_utils::*;

#[test]
fn test_registration_successful() {
    let new_user = r#"{
        "email_address": "mate@coolcat.com",
        "password": "supersecret123"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let (tx, rx) = channel();
    let (_eloop, web3) = get_web3();
    let handler = api::ApiHandler::new(&get_config(), web3, tx);
    let response = request::post(
        "http://localhost:3000/users/register",
        headers,
        new_user,
        &handler,
    ).unwrap();
    let result_body = response::extract_body_to_string(response);
    let reg_result: api::RegistrationInfo = serde_json::from_str(&result_body).unwrap();
    assert_eq!("mate@coolcat.com", reg_result.email_address);
    assert_eq!(20, reg_result.identifier_code.0.len());
}

#[test]
fn test_creates_new_user_and_password_works() {
    use chrono::prelude::*;

    let mut db = get_db_connection();
    let new_user = r#"{
        "email_address": "just.mate.antunovic@gmail.com",
        "password": "supersecret1234"
    }"#;
    let mut headers = Headers::new();
    headers.set_raw("content-type", vec![b"application/json".to_vec()]);
    let (tx, rx) = channel();
    let (_eloop, web3) = get_web3();
    let handler = api::ApiHandler::new(&get_config(), web3, tx);
    let register_response = request::post(
        "http://localhost:3000/users/register",
        headers.clone(),
        new_user,
        &handler,
    ).unwrap();

    let result_body = response::extract_body_to_string(register_response);
    let reg_result: api::RegistrationInfo = serde_json::from_str(&result_body).unwrap();
    let registration: domain::Registration = reg_result.identifier_code.get(&mut db).unwrap();

    let dob = NaiveDate::from_ymd(1999, 10, 1);

    let confirmation = api::RegistrationConfirm {
        email_address: "pat@coolcat.com".to_owned(),
        confirmation_code: registration.confirmation_code,
        identifier_code: registration.identifier_code,
        personal_details: api::PersonalDetails {
            first_names: "John Bambam Windsor".to_owned(),
            family_name: "Conovich".to_owned(),
            address_line_1: "55 Cuddlebear lane".to_owned(),
            address_line_2: None.to_owned(),
            post_code: "4231".to_owned(),
            city: "Wellington".to_owned(),
            country: "NEW ZEALAND".to_owned(),
            dob: dob,
            id_type: "Passport_NZ".to_owned(),
            id_number: "LM123456".to_owned(),
        },
        eth_account_password: "ethpassword123".to_owned(),
    };

    request::post(
        "http://localhost:3000/users/confirm",
        headers.clone(),
        &serde_json::to_string(&confirmation).unwrap(),
        &handler,
    ).unwrap();

    let login_response = request::post(
        "http://localhost:3000/users/log_in",
        headers.clone(),
        r#"{
            "email_address": "pat@coolcat.com",
            "password": "supersecret1234"
        }"#,
        &handler,
    ).unwrap();
    assert_eq!(Status::Ok, login_response.status.unwrap());
}

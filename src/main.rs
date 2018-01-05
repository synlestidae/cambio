extern crate bcrypt;
extern crate iron;
extern crate postgres;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate chrono;
extern crate time;

#[macro_use]
extern crate serde_derive;

mod db;
mod domain;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order, ApiError, Session};
use db::{PostgresHelperImpl, PostgresHelper, UserRepository};
use std::error::Error;

fn make_order(order: &Order, session_id: &str, email_address: &str) -> Result<Session, ApiError> {
    // stored procedure handles most of this
    let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None)
        .unwrap();
    let db = PostgresHelperImpl::new(conn);

    // this code is authorised
    unimplemented!()

}

fn log_in_user(unauthed_user: &User) -> Result<Session, ApiError> {
    let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None)
        .unwrap();
    let db = PostgresHelperImpl::new(conn);
    let mut user_repository = UserRepository::new(db);
    let password = match unauthed_user.password {
        None => {
            return Err(ApiError::missing_field_or_param(
                "Password was not supplied",
            ))
        }
        Some(ref password) => password.to_owned(),
    };
    let session_result = user_repository.log_user_in(&unauthed_user.email_address, password);

    match session_result {
        Ok(None) => Err(ApiError::invalid_login(
            "Invalid username and password combination",
        )),
        Err(error) => {
            let msg = format!("Error logging in: {}", error.description());
            Err(ApiError::invalid_login(&msg))
        }
        Ok(None) => {
            Err(ApiError::invalid_login(
                "Error logging in: account does not exist",
            ))
        }
        Ok(Some(session)) => Ok(session),
    }
}

fn main() {
    const MAX_BODY_LENGTH: usize = 1024 * 512; //max 512 Kb
    let mut router = Router::new();
    //router.post("/user/login", log_in_user, "login_user");
    Iron::new(router).http("localhost:3000").unwrap();
}

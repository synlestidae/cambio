extern crate bcrypt;
extern crate iron;
extern crate postgres;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate chrono;

#[macro_use] extern crate serde_derive;

mod db;
mod domain;

use iron::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};
use bcrypt::{DEFAULT_COST, hash, verify};
use postgres::{Connection, TlsMode};
use domain::{User, Order, ApiError, Session};
use db::{PostgresHelperImpl, PostgresHelper};
use std::error::Error;

fn log_in_user(unauthed_user: &User) -> Result<Session, ApiError> {  
    let user_obj_password = match unauthed_user.password {
        Some(ref password) => password.clone(),
        None => return Err(ApiError::missing_field_or_param("Missing password field"))
    };

    // let email_address = user_obj.email_address;
    let conn = Connection::connect("postgres://mate@localhost:5432/coin_boi", TlsMode::None).unwrap();
    let db = PostgresHelperImpl::new(conn);

    let user: User;
    let not_found_error = Err(ApiError::invalid_login("Failed to find your email address in the database"));
    let query_result = db.query("SELECT email_address, password_hash FROM users WHERE email_address = $1;", 
        &[&unauthed_user.email_address]);

    if let Ok(matching_users) = query_result {
        if let Some(user_match) = matching_users.pop() {
            user = user_match;
        } else {
            return not_found_error;
        }
    } else {
        return not_found_error;
    }

    // this should never happen, but we double check what the database gave us
    if (user.email_address != unauthed_user.email_address) {
        return not_found_error;
    }

    if (!user.hash_matches_password(&user_obj_password)) {
        return not_found_error;
    }

    // all code after this point is authorised. the user has proven their identity
    match db.query("SELECT session_token FROM activate_user_session($1);", &[&user.email_address]) {
        Ok(result) => {
            let session_token_error = Err(ApiError::query_result_format("Could not get your session token from the database"));
            for row in result.iter() {
                let session_token_option: Option<String>; 
                session_token_option = row.get(row, "session_token");

                if let Some(session_token) = session_token_option {
                    return Ok(Session {
                        session_token: session_token,
                        email_address: user.email_address,
                        expires_at: None
                    });
                }

                return session_token_error;
            }

            return session_token_error;
        },
        Err(error) => {
            let err_msg = format!("Could not connect to the database: {}", error.description());
            return Err(ApiError::database_driver(&err_msg));
        }
    }
}

fn main() {
    const MAX_BODY_LENGTH:usize = 1024 * 512; //max 512 Kb
    let mut router = Router::new();
    //router.post("/user/login", log_in_user, "login_user");
    Iron::new(router).http("localhost:3000").unwrap();
}

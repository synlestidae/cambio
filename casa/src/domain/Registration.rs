use domain::{RegistrationId, IdentifierCode};
use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::{TryFromRow, TryFromRowError};
use postgres;
use postgres::rows::Row;
use rand;
use rand::Rng;
use bcrypt::{hash};
use rand::distributions::Alphanumeric;
use std::iter;

#[derive(TryFromRow, Debug, Clone)]
pub struct Registration {
    pub id: Option<RegistrationId>,
    pub email_address: String,
    pub password_hash: String,
    pub confirmation_code: String, 
    pub identifier_code: IdentifierCode, 
    pub requested_at: NaiveDateTime,
    pub confirmed_at: Option<NaiveDateTime>
}

impl Registration {
    pub fn new(email_address: &str, password: &str) -> Self {
        const BCRYPT_COST: u32 = 8;
        let password_hash = hash(&password, BCRYPT_COST).unwrap();
        Self {
            id: None,
            email_address: email_address.to_owned(),
            password_hash: password_hash,
            confirmation_code: random_5_digit_code(),
            identifier_code: random_identifier_code(),
            requested_at: Utc::now().naive_utc(),
            confirmed_at: None
        }
    }

    pub fn confirm(&mut self) {
        self.confirmed_at = Some(Utc::now().naive_utc());
    }
}

fn random_identifier_code() -> IdentifierCode {
    let mut rng = rand::thread_rng();
    //let mut token = String::new();
    /*for _ in 0..20 {
       if rand::random::<bool>() {
           token.push_str(&rng.gen_range(0, 9).to_string());
       } else {
           token.push(rng.gen_range('a', 'z'));
       }
    }*/
    //token
    IdentifierCode(iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(20)
        .collect())
}

fn random_5_digit_code() -> String {
    let mut rng = rand::thread_rng();
    let mut token = String::new();
    for _ in 0..5 {
        token.push_str(&rng.gen_range(0, 9).to_string());
    }
    token
}
use chrono::prelude::*;
use chrono::Duration;
use chrono::prelude::{DateTime, Utc};
use db::{TryFromRow, TryFromRowError};
use domain::{IdentifierCode, RegistrationId};
use postgres;
use postgres::rows::Row;
use rand;
use event::EventKey;
use colectivo::MessageKey;

use bcrypt::hash;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::iter;

#[derive(TryFromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub id: Option<RegistrationId>,
    pub email_address: String,
    pub password_hash: String,
    pub confirmation_code: String,
    pub identifier_code: IdentifierCode,
    pub requested_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub confirmed_at: Option<DateTime<Utc>>,
}

impl Registration {
    pub fn new(email_address: &str, password: &str) -> Self {
        const BCRYPT_COST: u32 = 8;
        let password_hash = hash(&password, BCRYPT_COST).unwrap();
        let requested_at = Utc::now();
        Self {
            id: None,
            email_address: email_address.to_owned(),
            password_hash: password_hash,
            confirmation_code: random_5_digit_code(),
            identifier_code: random_identifier_code(),
            requested_at: requested_at,
            expires_at: requested_at + Duration::hours(4),
            is_active: true,
            confirmed_at: None,
        }
    }

    pub fn confirm(&mut self) {
        self.confirmed_at = Some(Utc::now());
    }
}

impl EventKey for Registration {
    fn key(&self) -> MessageKey {
        /*use byteorder::{LittleEndian, WriteBytesExt};
        let mut key_bytes = vec![];
        key_bytes.write_u16::<LittleEndian>(self.id.0).unwrap();
        MessageKey(key_bytes)*/
        MessageKey(serde_json::to_string(&self.id).unwrap().into_bytes())
    }
}

fn random_identifier_code() -> IdentifierCode {
    let mut rng = rand::thread_rng();
    IdentifierCode(
        iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(20)
            .collect(),
    )
}

fn random_5_digit_code() -> String {
    let mut rng = rand::thread_rng();
    let mut token = String::new();
    for _ in 0..5 {
        token.push_str(&rng.gen_range(0, 9).to_string());
    }
    token
}

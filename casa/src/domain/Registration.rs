use domain::RegistrationId;
use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::{TryFromRow, TryFromRowError};
use postgres;
use postgres::rows::Row;

#[derive(TryFromRow, Debug, Clone)]
pub struct Registration {
    pub id: Option<RegistrationId>,
    pub email_address: String,
    pub password_hash: String,
    pub confirmation_code: String, 
    pub identifier_code: String, 
    pub requested_at: DateTime<Utc>,
    pub confirmed_at: Option<DateTime<Utc>>
}

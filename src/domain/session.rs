use chrono::prelude::{DateTime, Utc};
use time::Duration;
use db::TryFromRow;
use db::TryFromRowError;
use std;
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub session_token: String,
    pub email_address: String,
    pub expires_at: DateTime<Utc>,
}

impl TryFromRow for Session {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError>
    where
        Self: std::marker::Sized,
    {
        let email_address_match: Option<String> = row.get("email_address");
        let session_token_match: Option<String> = row.get("session_token");
        let started_at_match: Option<DateTime<Utc>> = row.get("started_at");
        let ttl_milliseconds_match: Option<i64> = row.get("ttl_milliseconds");
        match (
            email_address_match,
            session_token_match,
            started_at_match,
            ttl_milliseconds_match,
        ) {
            (Some(email_address),
             Some(session_token),
             Some(started_at),
             Some(ttl_milliseconds)) => {
                let expiry_date = started_at + Duration::milliseconds(ttl_milliseconds);
                Ok(Session {
                    session_token: session_token,
                    email_address: email_address,
                    expires_at: expiry_date,
                })
            }
            _ => Err(TryFromRowError {}),
        }
    }
}

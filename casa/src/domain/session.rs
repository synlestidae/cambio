use chrono::prelude::{DateTime, Utc};
use time::Duration;
use db::TryFromRow;
use db::TryFromRowError;
use chrono::NaiveDateTime;
use std;
use postgres::rows::Row;
use postgres;
use domain::{Id, SessionState};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Session {
    pub id: Option<Id>,
    pub session_token: String,
    pub started_at: DateTime<Utc>,
    pub ttl_milliseconds: i32,
    pub email_address: Option<String>,
    pub session_state: SessionState
}

impl Session {
    pub fn is_valid(&self) -> bool {
        let duration = Duration::milliseconds(self.ttl_milliseconds as i64);
        Utc::now() < self.started_at + duration
    }
}

/*impl TryFromRow for Session {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError>
    where
        Self: std::marker::Sized,
    {
        let email_address_match: Option<String> = row.get("email_address");
        let session_token_match: Option<String> = row.get("session_token");
        let started_at_match: Option<NaiveDateTime> = row.get("started_at");
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
                Ok(Session {
                    session_token: session_token,
                    email_address: email_address,
                    expires_at: DateTime::from_utc(started_at, Utc) +
                        Duration::milliseconds(ttl_milliseconds),
                })
            }
            _ => Err(TryFromRowError::new(
                "At least one required field for 'Session' is missing",
            )),
        }
    }
}*/

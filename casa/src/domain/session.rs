use chrono::prelude::{DateTime, Utc};
use time::Duration;
use db::TryFromRow;
use db::TryFromRowError;
use chrono::NaiveDateTime;
use std;
use postgres::rows::Row;
use postgres;
use domain::{Id, SessionState, SessionToken};
use rand;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Session {
    pub id: Option<Id>,
    pub user_id: Id,
    pub session_token: SessionToken,
    pub started_at: DateTime<Utc>,
    pub ttl_milliseconds: i64,
    pub email_address: Option<String>,
    pub session_state: SessionState,
}

impl Session {
    pub fn new(email_address: &str, user_id: Id, ttl_milliseconds: i32) -> Self {
        Self {
            id: None,
            user_id: user_id,
            session_token: random_token_string(),
            started_at: Utc::now(),
            ttl_milliseconds: SESSION_TIME_MILLISECONDS,
            email_address: Some(email_address.to_owned()),
            session_state: SessionState::Valid,
        }
    }
    pub fn is_valid(&self) -> bool {
        let duration = Duration::milliseconds(self.ttl_milliseconds as i64);
        Utc::now() < self.started_at + duration
    }
}

fn random_token_string() -> SessionToken {
    let mut token = String::new();
    for _ in (0..10) {
        token.push(rand::random::<u8>() as char);
    }
    SessionToken(token)
}

const SESSION_TIME_MILLISECONDS: i64 = 1000 * 60 * 15;

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

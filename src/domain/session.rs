use chrono::prelude::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub session_token: String,
    pub email_address: String,
    pub expires_at: Option<DateTime<Utc>>
}

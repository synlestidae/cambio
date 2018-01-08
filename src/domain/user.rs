use db::TryFromRow;
use bcrypt::{verify};
use db::TryFromRowError;
use std;
use domain::Id;
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct User {
    pub id: Option<Id>,
    pub email_address: String,
    pub password: Option<String>,
    pub password_hash: Option<String>,
}

impl User {
    pub fn password_matches_hash(&self, hash: &str) -> bool {
        match self.password {
            Some(ref password) => verify(&password, &hash).is_ok(),
            _ => false,
        }
    }

    pub fn hash_matches_password(&self, password: &str) -> bool {
        match self.password_hash {
            Some(ref hash) => verify(&password, &hash).is_ok(),
            _ => false,
        }
    }
}

impl TryFromRow for User {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError>
    where
        Self: std::marker::Sized,
    {
        let email_address_match: Option<String> = row.get("email_address");
        let password_hash_match: Option<String> = row.get("password_hash");
        let id: Option<i32> = row.get("id");
        match (email_address_match, password_hash_match, id) {
            (Some(email_address), Some(password_hash), Some(id)) => {
                return Ok(User {
                    id: Some(id),
                    email_address: email_address,
                    password_hash: Some(password_hash),
                    password: None,
                });
            },
            _ => Err(TryFromRowError {}),
        }
    }
}

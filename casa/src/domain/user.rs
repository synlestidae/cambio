use bcrypt::{hash, verify};
use checkmail;
use db::TryFromRowError;
use db::{CambioError, TryFromRow};
use domain::{OwnerId, UserId};
use postgres::rows::Row;
use std;

const BCRYPT_COST: u32 = 8;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct User {
    #[column_id(user_id)]
    pub id: Option<UserId>,
    pub email_address: String,
    pub password: Option<String>,
    pub password_hash: Option<String>,
    pub owner_id: Option<OwnerId>,
}

impl User {
    pub fn new_register(email_address: &str, password: String) -> User {
        let password_hash = hash(&password, BCRYPT_COST).unwrap();
        User {
            id: None,
            email_address: email_address.to_owned(),
            password: None,
            password_hash: Some(hash(&password, BCRYPT_COST).unwrap()),
            owner_id: None,
        }
    }

    pub fn password_matches_hash(&self, hash: &str) -> bool {
        match self.password {
            Some(ref password) => verify(&password, &hash).is_ok(),
            _ => false,
        }
    }

    pub fn hash_matches_password(&self, password: &str) -> bool {
        match self.password_hash {
            Some(ref hash) => match verify(&password, &hash) {
                Ok(is_match) => is_match,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn change_password(&mut self, password: &str) {
        self.password = Some(hash(&password, BCRYPT_COST).unwrap());
    }
}

impl TryFromRow for User {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError>
    where
        Self: std::marker::Sized,
    {
        let email_address_match: Option<String> = row.get("email_address");
        let password_hash_match: Option<String> = row.get("password_hash");
        let id: Option<UserId> = row.get("id");
        let owner_id: Option<OwnerId> = row.get("owner_id");
        match (email_address_match, password_hash_match, id, owner_id) {
            (Some(email_address), Some(password_hash), Some(id), Some(oi)) => {
                return Ok(User {
                    id: Some(id),
                    email_address: email_address,
                    password_hash: Some(password_hash),
                    password: None,
                    owner_id: Some(oi),
                });
            }
            _ => Err(TryFromRowError::new(
                "A required field for 'User' is missing",
            )),
        }
    }
}

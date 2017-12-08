use serde_derive;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub email_address: String,
    pub password: Option<String>,
    pub password_hash: Option<String>
}

impl User {
    pub fn password_matches_hash(&self, hash: &str) -> bool {
        match self.password {
            Some(ref password) => verify(&password, &hash).is_ok(),
            _ => false
        }
    }

    pub fn hash_matches_password(&self, password: &str) -> bool {
        match self.password_hash{
            Some(ref hash) => verify(&password, &hash).is_ok(),
            _ => false
        }
    }
}

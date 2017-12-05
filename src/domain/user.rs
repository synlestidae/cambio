use serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub email_address: String,
    pub password: String
}

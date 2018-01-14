#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Registration {
    pub email_address: String,
    pub password: String,
}

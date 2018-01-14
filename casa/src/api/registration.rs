#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub email_address: String,
    pub password: String,
}

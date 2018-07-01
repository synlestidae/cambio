#[derive(Debug, Deserialize, Clone)]
pub struct ResendEmail {
    email_address: String,
    identifier_code: String
}

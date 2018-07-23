use domain::IdentifierCode;

#[derive(Debug, Deserialize, Clone)]
pub struct ResendEmail {
    pub email_address: String,
    pub identifier_code: IdentifierCode,
}

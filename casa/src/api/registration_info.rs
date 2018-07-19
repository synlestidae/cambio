use domain::IdentifierCode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistrationInfo {
    pub email_address: String,
    pub identifier_code: IdentifierCode,
}

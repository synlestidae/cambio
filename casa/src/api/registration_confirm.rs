use domain;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationConfirm {
    pub email_address: String,
    pub confirmation_code: String, 
    pub identifier_code: domain::IdentifierCode
}

impl RegistrationConfirm {
    pub fn can_confirm(&self, other: &domain::Registration) -> bool {
        return self.email_address == other.email_address &&
            self.confirmation_code == other.confirmation_code && 
            self.identifier_code == other.identifier_code;
    }
}
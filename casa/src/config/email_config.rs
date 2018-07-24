use lettre::EmailAddress;

#[derive(Clone)]
pub struct EmailConfig {
    pub login: String,
    pub email_address: EmailAddress,
    pub password: EmailAddress,
    pub server_host: EmailAddress,
}

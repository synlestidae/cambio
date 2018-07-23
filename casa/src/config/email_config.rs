use lettre::EmailAddress;

#[derive(Clone)]
pub struct EmailConfig {
    pub login: EmailAddress,
    pub email_address: EmailAddress,
    pub password: EmailAddress,
    pub server_host: EmailAddress,
}

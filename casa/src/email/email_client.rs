use config::EmailConfig;
use email::email_message::EmailMessage;
use email::smtp_response::SMTPResponse;
use email::email_client_error::EmailClientError;
use lettre::{EmailTransport, SmtpTransport};
use lettre::smtp::authentication::Credentials;

pub struct EmailClient {
    server_host: String,
    login: String,
    password: String
}

impl EmailClient {
    pub fn new(config: &EmailConfig) -> Self {
        Self {
            server_host: config.server_host.to_string(),
            login: config.login.to_string(),
            password: config.password.to_string() 
        }
    }

    pub fn send(&self, message: &EmailMessage) -> Result<SMTPResponse, EmailClientError> {
        let lettre_email = message.to_lettre_email();
        let credentials = Credentials::new(self.login.to_string(), self.password.to_string());
        let mut client = SmtpTransport::simple_builder(&self.server_host)?
            .build();
        let lettre_response = client.send(&lettre_email)?;
        let response = SMTPResponse::from_code(&lettre_response.code);
        if response.success() {
            Ok(response)
        } else {
            Err(response.into())
        }
    }
}

use config::EmailConfig;
use email::contact_spec::ContactSpec;
use email::email_client_error::EmailClientError;
use email::email_message::EmailMessage;
use email::smtp_response::SMTPResponse;
use email::to_email_message::ToEmailMessage;
use lettre::smtp::authentication::Credentials;
use lettre::smtp::client::net::ClientTlsParameters;
use lettre::smtp::ClientSecurity;
use lettre::{EmailTransport, SmtpTransport};

#[derive(Debug)]
pub struct EmailClient {
    server_host: String,
    login: String,
    password: String,
}

impl EmailClient {
    pub fn new(config: &EmailConfig) -> Self {
        Self {
            server_host: config.server_host.to_string(),
            login: config.login.to_string(),
            password: config.password.to_string(),
        }
    }

    pub fn send(&self, message: &EmailMessage) -> Result<SMTPResponse, EmailClientError> {
        let lettre_email = message.to_lettre_email();
        let mut client = self.get_transport()?;
        info!("Sending email from {} to {}", message.from, message.to);
        let lettre_response = client.send(&lettre_email)?;
        let response = SMTPResponse::from_code(&lettre_response.code);
        if response.success() {
            info!("Email was a success");
            Ok(response)
        } else {
            warn!("Email failed: {:?}", response);
            Err(response.into())
        }
    }

    fn get_transport(&self) -> Result<SmtpTransport, EmailClientError> {
        let credentials = Credentials::new(self.login.to_string(), self.password.to_string());
        info!("Preparing email transport to host {}", self.server_host);
        let builder = SmtpTransport::simple_builder(&self.server_host)?;
        let client = builder
            .credentials(credentials)
            .build();
        Ok(client)
    }
}

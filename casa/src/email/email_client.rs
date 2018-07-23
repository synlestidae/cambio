use config::EmailConfig;
use email::email_message::EmailMessage;
use email::smtp_response::SMTPResponse;
use email::email_client_error::EmailClientError;
use lettre::{EmailTransport, SmtpTransport};
use lettre::smtp::authentication::Credentials;
use email::to_email_message::ToEmailMessage;
use email::contact_spec::ContactSpec;
use lettre::smtp::client::net::ClientTlsParameters;
use lettre::smtp::ClientSecurity;

#[derive(Debug)]
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

    pub fn send<M: ToEmailMessage>(&self, contact: &ContactSpec, message_src: &M) -> Result<SMTPResponse, EmailClientError> {
        let message = message_src.to_email_message(contact);
        let lettre_email = message.to_lettre_email();
        let mut client = self.get_transport()?;
        let lettre_response = client.send(&lettre_email)?;
        let response = SMTPResponse::from_code(&lettre_response.code);
        if response.success() {
            Ok(response)
        } else {
            Err(response.into())
        }
    }

    fn get_transport(&self) -> Result<SmtpTransport, EmailClientError> {
        let credentials = Credentials::new(self.login.to_string(), self.password.to_string());
        let tls_params = ClientTlsParameters::new(self.server_host.clone(), unimplemented!());
        let security = ClientSecurity::Opportunistic(tls_params);
        let host = format!("{}:{}", self.server_host, "465");
        let builder = SmtpTransport::builder(&host, security)?;
        let client = builder.build();
        Ok(client)
    }
}

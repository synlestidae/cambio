use config::EmailConfig;
use email::email_message::EmailMessage;

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

    pub fn send(&self, message: &EmailMessage) -> Result<SMTPResponse, SMTPError> {
        unimplemented!()
    }
}

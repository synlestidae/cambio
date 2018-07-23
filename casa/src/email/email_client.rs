use config::EmailConfig;
use email::email_message::EmailMessage;

pub struct EmailClient {
    server_host: String,
    username: String,
    password: String
}

impl EmailClient {
    pub fn new(config: &EmailConfig) -> Self {
        unimplemented!()
    }

    pub fn send(&self, message: &EmailMessage) -> Result<SMTPResponse, SMTPError> {
        unimplemented!()
    }
}

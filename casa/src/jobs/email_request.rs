use lettre::EmailAddress;
//use email::ToEmailMessage;
use email::*;

#[derive(Debug)]
pub enum EmailRequest {
    ConfirmationCode {
        from: EmailAddress,
        to: EmailAddress,
        name: String,
        confirmation_code: String 
    }
}

impl EmailRequest {
    pub fn to_email(&self) -> EmailMessage {
        unimplemented!()
    }
}

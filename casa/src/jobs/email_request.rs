use lettre::EmailAddress;

#[derive(Debug)]
pub enum EmailRequest {
    ConfirmationCode {
        email_address: EmailAddress,
        confirmation_code: String 
    }
}

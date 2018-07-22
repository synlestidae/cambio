use lettre::EmailAddress;
use email::message_body::MessageBody;

pub struct EmailMessage {
    pub from: EmailAddress,
    pub from_name: Option<String>,
    pub to: EmailAddress,
    pub to_name: Option<String>,
    pub subject: String,
    pub body: MessageBody
}

use email::message_body::MessageBody;
use lettre::EmailAddress;
use lettre::SimpleSendableEmail;
use lettre_email::Email;
use lettre_email::EmailBuilder;

pub struct EmailMessage {
    pub from: EmailAddress,
    pub from_name: Option<String>,
    pub to: EmailAddress,
    pub to_name: Option<String>,
    pub subject: String,
    pub body: MessageBody,
}

impl EmailMessage {
    pub fn new_plain(from: &EmailAddress, to: &EmailAddress, subject: &str, body: &str) -> Self {
        Self {
            from: from.clone(),
            from_name: None,
            to: to.clone(),
            to_name: None,
            subject: subject.to_string(),
            body: MessageBody::PlainText(body.to_string()),
        }
    }

    pub fn to_lettre_email(&self) -> Email {
        EmailBuilder::new()
            .from(self.from.to_string())
            .to(self.to.to_string())
            .subject(self.subject.to_string())
            .text(match self.body {
                MessageBody::PlainText(ref text) => text.to_string(),
            })
            .build()
            .unwrap() // Should never fail
    }
}

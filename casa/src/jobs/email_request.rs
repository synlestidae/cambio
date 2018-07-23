use lettre::EmailAddress;
use email::*;

#[derive(Debug)]
pub enum EmailRequest {
    ConfirmationCode {
        from: EmailAddress,
        to: EmailAddress,
        name: String,
        confirmation_code: String,
    },
}

impl EmailRequest {
    pub fn confirmation_email(
        from: &EmailAddress,
        to: &EmailAddress,
        name: &str,
        code: &str,
    ) -> Self {
        EmailRequest::ConfirmationCode {
            from: from.clone(),
            to: to.clone(),
            name: name.to_owned(),
            confirmation_code: code.to_owned(),
        }
    }

    pub fn to_email(&self) -> EmailMessage {
        match self {
            &EmailRequest::ConfirmationCode {
                from: ref from,
                to: ref to,
                name: ref name,
                confirmation_code: ref code,
            } => {
                let confirm = ConfirmationRequestEmail::new(code, name);
                confirm.to_email_message(&ContactSpec::new_from_to(from, to))
            }
        }
    }
}

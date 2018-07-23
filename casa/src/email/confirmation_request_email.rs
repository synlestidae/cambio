use email::to_email_message::ToEmailMessage;
use email::contact_spec::ContactSpec;
use email::email_message::EmailMessage;
use email::message_body::MessageBody;

pub struct ConfirmationRequestEmail {
    confirmation_code: String,
    given_name: String
}

impl ConfirmationRequestEmail {
    pub fn new(confirmation_code: String, given_name: String) -> Self {
        Self {
            confirmation_code: confirmation_code,
            given_name: given_name
        }
    }

    fn get_subject(&self) -> String {
        format!("{} is your Cambio confirmation code", self.confirmation_code)
    }

    fn get_body(&self) -> String {
        format!("Hi {given_name},\r\n
\r\n
Your registration on Cambio.co.nz is almost confirmed. Just enter {confirmation_code} on the signup page to continue. If you didn't request this email, please ignore it as no action is required on your part.\r\n
\r\n
The Cambio team
", given_name=self.given_name, confirmation_code=self.confirmation_code)
    }
}

impl ToEmailMessage for ConfirmationRequestEmail {
    fn to_email_message(&self, contact: &ContactSpec) -> EmailMessage {
        EmailMessage::new_plain(&contact.from, 
            &contact.to, 
            &self.get_subject(), 
            &self.get_body())
    }
}

mod test {
    use email::email_message::*;
    use email::confirmation_request_email::*;
    use email::contact_spec::*;
    use lettre::EmailAddress;

    #[test]
    fn generates_subject_with_confirm_code() {
        let c = ConfirmationRequestEmail {
            confirmation_code: "12001".to_owned(),
            given_name: "Jhon Fernando".to_owned()
        };
        let contact = ContactSpec::new_from_to(
            &EmailAddress::new("noreply@cambio.co.nz".to_owned()).unwrap(),
            &EmailAddress::new("john@fernando.com".to_owned()).unwrap(),
        );
        let email = c.to_email_message(&contact);
        assert_eq!("12001 is your Cambio confirmation code", email.subject)
    }
}

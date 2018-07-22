use email::email_message::EmailMessage;
use email::contact_spec::ContactSpec;

pub trait ToEmailMessage {
    fn to_email_message(&self, contact: &ContactSpec) -> EmailMessage;
}

use email::contact_spec::ContactSpec;
use email::email_message::EmailMessage;

pub trait ToEmailMessage {
    fn to_email_message(&self, contact: &ContactSpec) -> EmailMessage;
}

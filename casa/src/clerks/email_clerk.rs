use event::Bus;
use domain::Registration;
use event::EventHandler;
use event::RegistrationEventType;
use config::EmailConfig;
use email::*;
use lettre::EmailAddress;
use std::error::Error;

pub struct EmailClerk {
    config: EmailConfig
}

impl EmailClerk {
    pub fn new(config: EmailConfig) -> Self {
        Self {
            config: config
        }
    }

    fn send_confirmation_email(&self, registration: Registration) {
        let email_request = ConfirmationRequestEmail::new(&registration.confirmation_code);
        let recp = EmailAddress::new(registration.email_address).unwrap(); // TODO this might panic one day
        let contact = ContactSpec::new_from_to(&self.config.email_address, &recp);
        let msg = email_request.to_email_message(&contact);

        let client = EmailClient::new(&self.config);    
        if let Err(err) = client.send(&msg) {
            warn!("Failed to send confirmation email to {}: {}", recp, err.description());
        }
    }
}

impl EventHandler for EmailClerk {
    type E = Registration;
    type Ty = RegistrationEventType;

    fn handle(&mut self, registration: Self::E, event_type: Self::Ty) {
        match event_type {
            RegistrationEventType::NewRegistration => {
                self.send_confirmation_email(registration);
            }
        }
    }
}

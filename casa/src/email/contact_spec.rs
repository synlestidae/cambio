use lettre::EmailAddress;

pub struct ContactSpec {
    pub from: EmailAddress,
    pub to: EmailAddress,
    pub from_name: Option<String>,
    pub to_name: Option<String>
}

impl ContactSpec {
    pub fn new_from_to(from: &EmailAddress, to: &EmailAddress) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            from_name: None,
            to_name: None
        }
    }
}

use lettre::EmailAddress;

pub struct ContactSpec {
    pub from_address: EmailAddress,
    pub to_address: EmailAddress,
    pub from_name: Option<String>,
    pub to_name: Option<String>
}

impl ContactSpec {
    pub fn new_from_to(from: &EmailAddress, to: &EmailAddress) -> Self {
        Self {
            from_address: from.clone(),
            to_address: to.clone(),
            from_name: None,
            to_name: None
        }
    }
}

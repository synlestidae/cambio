use lettre::smtp::error::Error as LettreError;

#[derive(Debug)]
pub struct SMTPError {
    can_retry: bool,
    pub original_error: LettreError,
}

impl From<LettreError> for SMTPError {
    fn from(err: LettreError) -> Self {
        let can_retry = match err {
            LettreError::Permanent(..) => false,
            LettreError::Client(..) => false,
            _ => true,
        };
        Self {
            can_retry: can_retry,
            original_error: err,
        }
    }
}

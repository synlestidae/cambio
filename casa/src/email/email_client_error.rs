use email::smtp_error::SMTPError;
use email::smtp_response::SMTPResponse;
use lettre::smtp::error::Error as LettreError;

#[derive(Debug)]
pub enum EmailClientError {
    SMTPError(SMTPError),
    SMTPFail(SMTPResponse),
    Net,
}

impl From<LettreError> for EmailClientError {
    fn from(err: LettreError) -> Self {
        EmailClientError::SMTPError(err.into())
    }
}

impl From<SMTPResponse> for EmailClientError {
    fn from(err: SMTPResponse) -> Self {
        EmailClientError::SMTPFail(err)
    }
}

impl From<SMTPError> for EmailClientError {
    fn from(err: SMTPError) -> Self {
        EmailClientError::SMTPError(err)
    }
}

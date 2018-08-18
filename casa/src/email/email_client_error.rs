use email::smtp_error::SMTPError;
use email::smtp_response::SMTPResponse;
use lettre::smtp::error::Error as LettreError;
use std::error::Error;
use std::fmt;

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

impl fmt::Display for EmailClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for EmailClientError {
    fn description(&self) -> &str {
        match self {
            EmailClientError::SMTPError(err) => err.original_error.description(),
            EmailClientError::SMTPFail(err) => "Error code over SMTP",
            EmailClientError::Net => "Network failure",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

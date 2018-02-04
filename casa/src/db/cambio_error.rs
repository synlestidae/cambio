use db;
use db::{ErrorKind, ErrorReccomendation};
use std::error;
use std::error::Error;
use std::fmt;
use web3;
use postgres;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CambioError {
    user_message: String,
    system_message: String,
    kind: ErrorKind,
    reccomendation: ErrorReccomendation,
}

impl CambioError {
    pub fn new(user_msg: &str, sys_msg: &str, kind: ErrorKind, recc: ErrorReccomendation) -> Self {
        Self {
            user_message: user_msg.to_owned(), 
            system_message: sys_msg.to_owned(), 
            kind: kind,
            reccomendation: recc
        }
    }

    pub fn user_exists() -> Self {
        Self {
            user_message: "This user is already registered. Please log in.".to_owned(),
            system_message: "User already exists in DB".to_owned(),
            kind: ErrorKind::UserExists,
            reccomendation: ErrorReccomendation::Nothing
        }
    }

    pub fn invalid_password() -> Self {
        Self {
            user_message: "Wrong password.".to_owned(),
            system_message: "BCrypt password doesn't match hash".to_owned(),
            kind: ErrorKind::Unauthorised,
            reccomendation: ErrorReccomendation::CheckInput
        }
    }
}

impl error::Error for CambioError {
    fn description(&self) -> &str {
        &self.system_message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for CambioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBHelperError: {}", self.description())
    }
}

impl From<web3::Error> for CambioError {
    fn from(err: web3::Error) -> CambioError {
        match err {
        }
    }
}

impl From<db::TryFromRowError> for CambioError {
    fn from(err: db::TryFromRowError) -> CambioError {
        unimplemented!()
    }
}

impl From<postgres::Error> for CambioError {
    fn from(err: postgres::Error) -> CambioError {
        unimplemented!()
    }
}

impl From<bcrypt::BcryptError> for CambioError {
    fn from(err: bcrypt::BcryptError) -> CambioError {
        unimplemented!()
    }
}

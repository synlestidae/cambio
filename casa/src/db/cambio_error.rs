use db;
use db::{ErrorKind, ErrorReccomendation};
use std::error;
use std::error::Error;
use std::fmt;
use web3;
use postgres;
use bcrypt;
use r2d2;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CambioError {
    pub user_message: String,
    pub system_message: String,
    pub kind: ErrorKind,
    pub reccomendation: ErrorReccomendation,
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

    pub fn bad_input(user_msg: &str, system_msg: &str) -> Self {
        Self {
            user_message: user_msg.to_owned(),
            system_message: system_msg.to_owned(),
            kind: ErrorKind::UserInputFormat,
            reccomendation: ErrorReccomendation::CheckInput
        }
    }

    pub fn shouldnt_happen(user_msg: &str, system_msg: &str) -> Self {
        Self {
            user_message: user_msg.to_owned(),
            system_message: system_msg.to_owned(),
            kind: ErrorKind::UnexpectedState,
            reccomendation: ErrorReccomendation::ContactProgrammer
        }
    }

    pub fn not_found_search(user_msg: &str, system_msg: &str) -> Self {
        Self {
            user_message: user_msg.to_owned(),
            system_message: system_msg.to_owned(),
            kind: ErrorKind::NotFound,
            reccomendation: ErrorReccomendation::CheckInput
        }
    }

    pub fn unfair_operation(user_msg: &str, system_msg: &str) -> Self {
        Self {
            user_message: user_msg.to_owned(),
            system_message: system_msg.to_owned(),
            kind: ErrorKind::UnfairOperation,
            reccomendation: ErrorReccomendation::Nothing
        }
    }

    pub fn format_obj(user_msg: &str, system_msg: &str) -> Self {
        Self {
            user_message: user_msg.to_owned(),
            system_message: system_msg.to_owned(),
            kind: ErrorKind::FormatObjInternal,
            reccomendation: ErrorReccomendation::CheckState
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
        CambioError {
            user_message: "Failed to communicate with Ethereum".to_owned(),
            system_message: format!("Web3 failed: {:?}", err),
            kind: ErrorKind::Web3,
            reccomendation: ErrorReccomendation::ContactProgrammer
        }
    }
}

impl From<db::TryFromRowError> for CambioError {
    fn from(err: db::TryFromRowError) -> CambioError {
        CambioError {
            user_message: "Something went wrong while converting internal data".to_owned(),
            system_message: format!("TryFromRowError: {}", err),
            kind: ErrorKind::ConvertingObjInternal,
            reccomendation: ErrorReccomendation::ContactProgrammer
        }
    }
}

impl From<postgres::Error> for CambioError {
    fn from(err: postgres::Error) -> CambioError {
        CambioError {
            user_message: "Failed to connect to the database".to_owned(),
            system_message: format!("Postgres error: {:?}", err),
            kind: ErrorKind::DBConnection,
            reccomendation: ErrorReccomendation::TryAgainNow
        }
    }
}

impl From<bcrypt::BcryptError> for CambioError {
    fn from(err: bcrypt::BcryptError) -> CambioError {
        CambioError {
            user_message: "Failed to create your account".to_owned(),
            system_message: format!("Bcrypt error {:?}", err),
            kind: ErrorKind::UnexpectedState,
            reccomendation: ErrorReccomendation::ContactProgrammer
        }
    }
}

impl From<r2d2::Error> for CambioError {
    fn from(err: r2d2::Error) -> CambioError {
        CambioError {
            user_message: "Failed to connect to the database".to_owned(),
            system_message: format!("r2d2 error: {:?}", err),
            kind: ErrorKind::DBConnection,
            reccomendation: ErrorReccomendation::TryAgainNow
        }
    }
}

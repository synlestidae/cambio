use payment::poli::{InitiateTransactionError, PoliErrorCode};
use db::{PostgresHelper, CambioError};
use std::error::Error;
use serde_xml_rs::Error as SerdeError;
use hyper::Error as HyperError;
use postgres::GenericConnection;
use domain::UserId;
use std::fmt;
use std::marker;

#[derive(Debug)]
pub enum PoliError {
    Request(Box<Error>),
    Response(Box<Error>),
    PoliError(PoliErrorCode),
    InitTx(Option<InitiateTransactionError>)
}

impl fmt::Display for PoliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for PoliError {
    fn description(&self) -> &str {
        match self {
            PoliError::Request(_) => "Creating the request for POLi failed",
            PoliError::Response(_) => "There was a fatal error in the POLi response",
            PoliError::PoliError(_) => "POLi returned an error code in its response",
            PoliError::InitTx(_) => "Could not initialised the transaction with POLi"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<Option<InitiateTransactionError>> for PoliError {
    fn from(errs: Option<InitiateTransactionError>) -> Self {
        PoliError::InitTx(errs)
    }
}

impl From<SerdeError> for PoliError {
    fn from(e: SerdeError) -> Self {
        PoliError::Request(Box::new(e))
    }
}

impl From<HyperError> for PoliError {
    fn from(e: HyperError) -> Self {
        PoliError::Response(Box::new(e))
    }
}

impl PoliError {
    pub fn save_in_log<C: GenericConnection>(&self, user_id: &Option<UserId>, db: &mut C) -> Result<(), CambioError> {
        Ok(())
    }
}

impl Into<CambioError> for PoliError {
    fn into(self) -> CambioError {
        unimplemented!()
    }
}

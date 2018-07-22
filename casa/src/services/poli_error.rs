use db::{CambioError, PostgresHelper};
use domain::UserId;
use hyper::Error as HyperError;
use payment::poli::{InitiateTransactionError, PoliErrorCode};
use postgres::GenericConnection;
use serde_json::error::Error as SerializeError;
use std::error::Error;
use std::fmt;
use std::marker;

#[derive(Debug)]
struct PoliErrorInfo {
    description: String,
}

#[derive(Debug)]
pub enum PoliError {
    Request(PoliErrorInfo),
    Response(PoliErrorInfo),
    PoliError(PoliErrorCode),
    InitTx(InitiateTransactionError),
    InitTxUnknown
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
            PoliError::InitTx(_) => "Could not initialise Poli transaction",
            PoliError::InitTxUnknown => "Could not initialise Poli transaction but an error code was not specified."
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<Option<InitiateTransactionError>> for PoliError {
    fn from(err: Option<InitiateTransactionError>) -> Self {
        match err {
            Some(e) => PoliError::InitTx(e),
            None => PoliError::InitTxUnknown
        }
    }
}

/*impl From<SerializeError> for PoliError {
    fn from(e: SerializeError) -> Self {
        PoliError::Request(PoliErrorInfo {
            description: e.description()
        })
    }
}*/

impl From<SerializeError> for PoliError {
    fn from(e: SerializeError) -> Self {
        PoliError::Request(PoliErrorInfo {
            description: e.description().to_owned(),
        })
    }
}

impl From<HyperError> for PoliError {
    fn from(e: HyperError) -> Self {
        PoliError::Response(PoliErrorInfo {
            description: e.description().to_owned(),
        })
    }
}

impl PoliError {
    pub fn save_in_log<C: GenericConnection>(
        &self,
        user_id: &Option<UserId>,
        db: &mut C,
    ) -> Result<(), CambioError> {
        Ok(())
    }
}

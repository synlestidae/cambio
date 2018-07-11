use payment::poli::{InitiateTransactionError, PoliErrorCode};
use db::{PostgresHelper, CambioError};
use std::error::Error;
use serde_xml_rs::Error as SerdeError;
use hyper::Error as HyperError;
use postgres::Connection;
use domain::UserId;

#[derive(Debug)]
pub enum PoliError {
    Request(Box<Error>),
    Response(Box<Error>),
    PoliError(PoliErrorCode),
    InitTx(Vec<InitiateTransactionError>)
}

impl From<Vec<InitiateTransactionError>> for PoliError {
    fn from(errs: Vec<InitiateTransactionError>) -> Self {
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
    pub fn save_in_log(&self, user_id: UserId, db: &mut Connection) -> Result<(), CambioError> {
        Ok(())
    }
}

impl Into<CambioError> for PoliError {
    fn into(self) -> CambioError {
        unimplemented!()
    }
}

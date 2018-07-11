use payment::poli::PoliErrorCode;
use std::error::Error;
use serde_xml_rs::Error as SerdeError;
use hyper::Error as HyperError;

#[derive(Debug)]
pub enum PoliError {
    Request(Box<Error>),
    Response(Box<Error>),
    PoliError(PoliErrorCode)
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

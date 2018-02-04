use std::error;
use std::error::Error;
use std::fmt;
use web3;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CambioError {
    desc: String,
}

impl CambioError {
    pub fn new(desc: &str) -> Self {
        Self { desc: desc.to_owned() }
    }
}

impl error::Error for CambioError {
    fn description(&self) -> &str {
        &self.desc
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
        CambioError::new(&format!("Error completing web3 operation: {:?}", err))
    }
}

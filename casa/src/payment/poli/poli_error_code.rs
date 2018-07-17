use payment::poli::PoliErrorCodeType;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct PoliErrorCode(pub String);

impl PoliErrorCode {
    pub fn get_type(&self) -> Option<PoliErrorCodeType> {
        let val = match u32::from_str(&self.0) {
            Ok(v) => v,
            _ => return None
        };
        let err_type = if 1001 <= val && val <= 1032 {
            PoliErrorCodeType::InitiateTransactionFailed
        } else if (2001 <= val && val <= 2027) || (2029 <= val && val <= 2038) {
            PoliErrorCodeType::PoliTransactionAbort
        } else if 3001 <= val && val <= 3030 {
            PoliErrorCodeType::PoliTransactionPageAbort
        } else if (5001 <= val && val <= 5002) || val == 5004 || (5006 <= val && val <= 5007) || (5010 <= val && val <= 5026) {
            PoliErrorCodeType::VectorError
        } else if val == 2028 || val == 5003 || (5008 <= val && val <= 5009) {
            PoliErrorCodeType::InvalidCertificate
        } else if val == 5005 {
            PoliErrorCodeType::UnexpectedBankPage
        } else if 6001 <= val && val <= 6005 {
            PoliErrorCodeType::CustomerUnable
        } else if 8001 <= val && val <= 8007 {
            PoliErrorCodeType::WebServiceError
        } else if 10001 <= val && val <= 10009 {
            PoliErrorCodeType::PaymentDataIncorrect
        } else if 12001 <= val && val <= 12024 {
            PoliErrorCodeType::PaymentAPIError
        } else {
            return None;
        };
        Some(err_type)
    }
}

impl fmt::Display for PoliErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for PoliErrorCode {
    fn description(&self) -> &str {
        "An error occurred while interacting with Poli"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl<'de> Deserialize<'de> for PoliErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = try!(String::deserialize(deserializer));
        Ok(PoliErrorCode(data))
    }
}

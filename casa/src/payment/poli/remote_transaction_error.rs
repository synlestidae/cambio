use payment::poli::*;

pub struct RemoteTransactionError {
    pub error_code: PoliErrorCode,
    pub error_message: Option<String>,
}

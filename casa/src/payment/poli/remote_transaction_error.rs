use payment::poli::*;

#[derive(Debug)]
pub struct RemoteTransactionError {
    pub error_code: PoliErrorCode,
    pub error_message: Option<String>,
}

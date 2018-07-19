#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PoliErrorCodeType {
    InitiateTransactionFailed,
    PoliTransactionAbort,
    PoliTransactionPageAbort,
    VectorError,
    InvalidCertificate,
    UnexpectedBankPage,
    CustomerUnable,
    WebServiceError,
    PaymentDataIncorrect,
    PaymentAPIError,
}

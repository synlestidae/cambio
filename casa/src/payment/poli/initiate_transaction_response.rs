use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionResponse {
    pub errors: Vec<InitiateTransactionError>,
    pub transaction_status_code: TransactionStatusCode,
    pub transaction: PoliTransactionResponse
}

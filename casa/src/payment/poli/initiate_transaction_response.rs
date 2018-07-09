use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionResponse {
    pub errors: Option<Vec<InitiateTransactionError>>,
    pub transaction_status_code: Option<TransactionStatusCode>,
    pub transaction: Option<PoliTransactionResponse>
}

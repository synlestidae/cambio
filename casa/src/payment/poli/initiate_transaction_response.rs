use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionResponse {
    #[serde(rename="Errors")]
    pub errors: Option<Vec<InitiateTransactionError>>,
    #[serde(rename="TransactionStatusCode")]
    pub transaction_status_code: Option<TransactionStatusCode>,
    #[serde(rename="Transaction")]
    pub transaction: Option<PoliTransactionResponse>
}

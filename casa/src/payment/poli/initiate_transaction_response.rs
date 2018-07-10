use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionResponse {
    #[serde(rename="Error", default)]
    pub errors: Vec<InitiateTransactionError>,
    #[serde(rename="TransactionStatusCode, default")]
    pub transaction_status_code: Option<TransactionStatusCode>,
    #[serde(rename="Transaction", default)]
    pub transaction: Vec<PoliTransactionResponse>
}

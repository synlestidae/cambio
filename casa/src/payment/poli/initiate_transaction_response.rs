use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitiateTransactionResponse {
    #[serde(rename="Error", default)]
    pub errors: Vec<InitiateTransactionError>,
    #[serde(rename="TransactionStatusCode")]
    pub transaction_status_code: Option<TransactionStatusCode>,
    #[serde(rename="Transaction", default)]
    pub transaction: Vec<PoliTransactionResponse>
}

impl InitiateTransactionResponse {
    pub fn get_transaction(mut self) 
        -> Result<PoliTransactionResponse, Vec<InitiateTransactionError>> {
        match (self.transaction.pop(), self.errors) {
            (Some(tx), _) => Ok(tx),
            (_, errors) => Err(errors)
        }
    }
}

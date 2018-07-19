use payment::poli::*;

#[derive(Serialize, Deserialize)]
pub struct GetTransaction {
    #[serde(rename = "TransactionToken")]
    pub transaction_token: TransactionToken,
}

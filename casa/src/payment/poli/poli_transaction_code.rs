use payment::poli::{TransactionToken, TransactionRefNo};

#[derive(Serialize, Deserialize)]
pub struct PoliTransactionResponse {
    #[serde(rename="TransactionToken")]
    pub transaction_token: TransactionToken,
    #[serde(rename="TransactionRefNo")]
    pub transaction_ref_no: TransactionRefNo
}

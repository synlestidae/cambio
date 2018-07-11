use payment::poli::{TransactionRefNo, TransactionToken};

#[derive(Serialize, Deserialize, Debug)]
pub struct PoliTransactionResponse {
    #[serde(rename="NavigateURL")]
    pub navigate_url: String,
    #[serde(rename="TransactionRefNo")]
    pub transaction_ref_no: TransactionRefNo,
    #[serde(rename="TransactionToken")]
    pub transaction_token: TransactionToken
}

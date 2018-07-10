use payment::poli::{TransactionRefNo, TransactionToken};

#[derive(Serialize, Deserialize)]
pub struct PoliTransactionResponse {
    pub navigate_url: String,
    pub transaction_ref_no: TransactionRefNo,
    pub transaction_token: TransactionToken
}

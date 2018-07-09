use payment::poli::{TransactionToken, TransactionRefNo};

#[derive(Serialize, Deserialize)]
pub struct PoliTransactionResponse {
    pub transaction_token: TransactionToken,
    pub transaction_ref_no: TransactionRefNo
}

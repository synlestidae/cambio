use payment::poli::{TransactionRefNo, TransactionToken};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PoliTransactionResponse {
    pub transaction_ref_no: TransactionRefNo,
    pub navigate_url: String,
}

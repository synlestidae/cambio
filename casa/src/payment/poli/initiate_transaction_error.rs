use payment::poli::PoliErrorCode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct InitiateTransactionError {
    pub code: PoliErrorCode,
    pub field: Option<String>,
    pub message: String
}

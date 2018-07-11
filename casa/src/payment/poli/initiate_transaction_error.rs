use payment::poli::PoliErrorCode;

#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionError {
    #[serde(rename="Code")]
    code: PoliErrorCode,
    #[serde(rename="Field")]
    field: Option<String>,
    #[serde(rename="Message")]
    message: String
}

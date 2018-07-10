#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionError {
    #[serde(rename="Code")]
    code: String,
    #[serde(rename="Field")]
    field: Option<String>,
    #[serde(rename="Message")]
    message: String
}

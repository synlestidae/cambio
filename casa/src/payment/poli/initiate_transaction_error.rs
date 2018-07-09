#[derive(Serialize, Deserialize)]
pub struct InitiateTransactionError {
    code: String,
    field: Option<String>,
    message: String
}

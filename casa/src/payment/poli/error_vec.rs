use payment::poli::InitiateTransactionError;

#[derive(Serialize, Deserialize)]
pub struct ErrorVec {
    #[serde(rename="Error")]
    errors: Vec<InitiateTransactionError>
}

impl Default for ErrorVec {
    fn default() -> Self {
        ErrorVec {
            errors: vec![]
        }
    }
}

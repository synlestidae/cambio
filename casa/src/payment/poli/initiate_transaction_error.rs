use payment::poli::PoliErrorCode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct InitiateTransactionError {
    code: PoliErrorCode,
    field: Option<String>,
    message: String
}

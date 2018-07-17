use payment::poli::PoliErrorCode;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct InitiateTransactionError {
	pub error_code: PoliErrorCode,
	pub error_message: String	
}

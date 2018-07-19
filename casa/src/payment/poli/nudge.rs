use payment::poli::*;

#[derive(Deserialize, Serialize)]
pub struct Nudge {
    #[serde(rename = "Token")]
    pub token: TransactionToken,
}

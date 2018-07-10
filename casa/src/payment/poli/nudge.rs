use payment::poli::*;

#[derive(Deserialize, Serialize)]
pub struct Nudge {
    pub token: TransactionToken
}

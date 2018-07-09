use payment::poli::PoliTransaction as Transaction;
use payment::poli::{AuthenticationCode};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitiateTransaction {
    #[serde(rename="AuthenticationCode")]
    pub authentication_code: AuthenticationCode,
    #[serde(rename="Transaction")]
    pub transaction: Transaction
}


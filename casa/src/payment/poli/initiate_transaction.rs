use payment::poli::PoliTransaction as Transaction;
use payment::poli::{AuthenticationCode};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransaction {
    #[serde(rename="AuthenticationCode")]
    authentication_code: AuthenticationCode,
    #[serde(rename="Transaction")]
    transaction: Transaction
}


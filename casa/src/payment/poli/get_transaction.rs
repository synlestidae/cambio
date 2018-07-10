use payment::poli::*;

#[derive(Serialize, Deserialize)]
pub struct GetTransaction {
    #[serde(rename="MerchantCode")]
    pub merchant_code: MerchantCode,
    #[serde(rename="AuthenticationCode")]
    pub authentication_code: AuthenticationCode,
    #[serde(rename="TransactionToken")]
    pub transaction_token: TransactionToken
}

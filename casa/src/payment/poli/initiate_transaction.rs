use payment::poli::{PoliConfig, AuthenticationCode, PoliTransaction as Transaction, MerchantRef};
use domain::PoliPaymentRequest;
use domain::CurrencyCode;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitiateTransaction {
    #[serde(rename="AuthenticationCode")]
    pub authentication_code: AuthenticationCode,
    #[serde(rename="Transaction")]
    pub transaction: Transaction
}

impl InitiateTransaction {
    pub fn from_request(poli_config: &PoliConfig, poli_payment_request: &PoliPaymentRequest) -> Self {
        let merchant_ref = MerchantRef(format!("Cambio Ltd|{}|Cred acc", poli_payment_request.unique_code));
        let transaction = Transaction {
            merchant_code: poli_config.merchant_code.clone(),
            currency_code: CurrencyCode::NZD,
            currency_amount: poli_payment_request.amount.clone(),
            merchant_date_time: Utc::now().naive_utc(),
            successful_url: poli_config.successful_url.to_string(),
            merchant_ref: Some(merchant_ref),
            merchant_data: None,
            selected_fi_code: None,
            notification_url: Some(poli_config.notification_url.to_string()),
            unsuccessful_url: Some(poli_config.unsuccessful_url.to_string()),
            merchant_checkout_url: Some(poli_config.merchant_checkout_url.to_string()),
            timeout: "3000".to_owned(),
            user_ip_address: None
        };
        InitiateTransaction {
            authentication_code: poli_config.authentication_code.clone(),
            transaction: transaction
        }
    }
}

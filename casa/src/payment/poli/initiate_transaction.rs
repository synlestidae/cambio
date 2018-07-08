use chrono::prelude::*;
use domain::{CurrencyCode, Decimal};
use payment::poli::{MerchantCode, AuthenticationCode, MerchantRef, MerchantData};

#[derive(Serialize, Deserialize)]
pub struct InitiateTransaction {
    merchant_code: MerchantCode,
    authentication_code: AuthenticationCode,
    merchant_ref: MerchantRef,
    merchant_data: MerchantData,
    currency_code: CurrencyCode,
    currency_amount: Decimal,
    merchant_date_time: NaiveDateTime,
    selected_fi_code: Option<String>,
    notification_url: Option<String>,
    successful_url: String,
    unsuccessful_url: Option<String>,
    merchant_checkout_url: Option<String>,
    timeout: u32,
    user_ip_address: String
}


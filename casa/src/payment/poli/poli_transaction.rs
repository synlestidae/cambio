use chrono::prelude::*;
use payment::poli::{MerchantRef, MerchantData, MerchantCode};
use domain::{CurrencyCode, Decimal};

#[derive(Serialize, Deserialize)]
pub struct PoliTransaction {
    #[serde(rename="MerchantCode")]
    merchant_code: MerchantCode,
    #[serde(rename="MerchantRef")]
    merchant_ref: MerchantRef,
    #[serde(rename="MerchantData")]
    merchant_data: MerchantData,
    #[serde(rename="CurrencyCode")]
    currency_code: CurrencyCode,
    #[serde(rename="CurrencyAmount")]
    currency_amount: Decimal,
    #[serde(rename="MerchantDateTime")]
    merchant_date_time: NaiveDateTime,
    #[serde(rename="SelectedFi_code")]
    selected_fi_code: Option<String>,
    #[serde(rename="NotificationUrl")]
    notification_url: Option<String>,
    #[serde(rename="SuccessfulURL")]
    successful_url: String,
    #[serde(rename="UnsuccessfulURL")]
    unsuccessful_url: Option<String>,
    #[serde(rename="MerchantCheckoutURL")]
    merchant_checkout_url: Option<String>,
    #[serde(rename="Timeout")]
    timeout: String,
    #[serde(rename="UserIPAddress")]
    user_ip_address: String 
}

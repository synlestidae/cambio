use chrono::prelude::*;
use domain::{CurrencyCode, Decimal};
use payment::poli::{MerchantCode, MerchantData, MerchantRef};

#[derive(Serialize, Deserialize, Debug)]
pub struct PoliTransaction {
    #[serde(rename = "MerchantCode")]
    pub merchant_code: MerchantCode,
    #[serde(rename = "CurrencyCode")]
    pub currency_code: CurrencyCode,
    #[serde(rename = "CurrencyAmount")]
    pub currency_amount: Decimal,
    #[serde(rename = "MerchantDateTime")]
    pub merchant_date_time: NaiveDateTime,
    #[serde(rename = "SuccessfulURL")]
    pub successful_url: String,
    #[serde(rename = "MerchantRef")]
    pub merchant_ref: Option<MerchantRef>,
    #[serde(rename = "MerchantData")]
    pub merchant_data: Option<MerchantData>,
    #[serde(rename = "SelectedFICode")]
    pub selected_fi_code: Option<String>,
    #[serde(rename = "NotificationURL")]
    pub notification_url: Option<String>,
    #[serde(rename = "UnsuccessfulURL")]
    pub unsuccessful_url: Option<String>,
    #[serde(rename = "MerchantCheckoutURL")]
    pub merchant_checkout_url: Option<String>,
    #[serde(rename = "Timeout")]
    pub timeout: String,
    #[serde(rename = "UserIPAddress")]
    pub user_ip_address: Option<String>,
}

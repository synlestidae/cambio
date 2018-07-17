use payment::poli::*;
use domain::PoliPaymentRequest;
use domain::CurrencyCode;
use domain::Decimal;
use chrono::prelude::*;

#[derive(Serialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct InitiateTransaction {
    amount: Decimal,
    currency_code: CurrencyCode,
    merchant_reference: MerchantRef,
    merchant_reference_format: Option<String>,
    merchant_data: Option<MerchantData>,
    #[serde(rename="MerchantHomepageURL")]
    merchant_homepage_url: String,
    #[serde(rename="SuccessURL")]
    success_url: String,
    #[serde(rename="FailureURL")]
    failure_url: Option<String>,
    #[serde(rename="CancellationURL")]
    cancellation_url: Option<String>,
    #[serde(rename="NotificationURL")]
    notification_url: Option<String>,
    timeout: Option<u32>,
    selected_fi_code: Option<String>
}

impl InitiateTransaction {
    pub fn from_request(poli_config: &PoliConfig, poli_payment_request: &PoliPaymentRequest) -> Self {
        let merchant_ref = MerchantRef(format!("Cambio Ltd|{}|Cred acc", poli_payment_request.unique_code));
        Self {
            amount: poli_payment_request.amount,
            currency_code: CurrencyCode::NZD,
            merchant_reference: merchant_ref,
            merchant_reference_format: Some(String::from("1")),
            merchant_data: None,
            merchant_homepage_url: poli_config.merchant_home_page_url.to_string(),
            success_url: poli_config.successful_url.to_string(),
            failure_url: Some(poli_config.unsuccessful_url.to_string()), 
            cancellation_url: None,
            notification_url: Some(poli_config.notification_url.to_string()),
            timeout: None,
            selected_fi_code: None
        }
    }
}

mod test {
    use payment::poli::*; 
    use domain::CurrencyCode;
    use serde_json::*;
    use domain::Decimal;
    use chrono::prelude::*;

    #[test]
    fn test_request_serializes() {
        let init_tx = InitiateTransaction {
            amount: Decimal::from_cents(120),
            currency_code: CurrencyCode::NZD,
            merchant_reference: MerchantRef("CustomerRef12345".to_owned()),
            merchant_reference_format: None,
            merchant_data: None,
            merchant_homepage_url: "https://www.mycompany.com".to_owned(),
            success_url: "https://www.mycompany.com/Success".to_owned(),
            failure_url: Some("https://www.mycompany.com/Failure".to_owned()),
            cancellation_url: Some("https://www.mycompany.com/Cancelled".to_owned()),
            notification_url: Some("https://www.mycompany.com/nudge".to_owned()),
            timeout: Some(3000),
            selected_fi_code: None
        };
        let map: Map<String, Value> = from_str(r#"{
            "Amount":"1.2",
            "CurrencyCode":"AUD",
            "MerchantReference":"CustomerRef12345",
            "MerchantHomepageURL":"https://www.mycompany.com",
            "SuccessURL":"https://www.mycompany.com/Success",
            "FailureURL":"https://www.mycompany.com/Failure",
            "CancellationURL":"https://www.mycompany.com/Cancelled",
            "NotificationURL":"https://www.mycompany.com/nudge"}
        "#).unwrap();

        assert_eq!("1.2", map["Amount"]);
        assert_eq!("AUD", map["CurrencyCode"]);
        assert_eq!("CustomerRef12345", map["MerchantReference"]);
        assert_eq!("https://www.mycompany.com", map["MerchantHomepageURL"]);
        assert_eq!("https://www.mycompany.com/Success", map["SuccessURL"]);
        assert_eq!("https://www.mycompany.com/Failure", map["FailureURL"]);
        assert_eq!("https://www.mycompany.com/Cancelled", map["CancellationURL"]);
        assert_eq!("https://www.mycompany.com/nudge", map["NotificationURL"]);
    }
}

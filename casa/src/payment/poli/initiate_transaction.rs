use payment::poli::{PoliConfig, AuthenticationCode, PoliTransaction as Transaction, MerchantRef};
use domain::PoliPaymentRequest;
use domain::CurrencyCode;
use chrono::prelude::*;

#[derive(Serialize, Debug)]
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

mod test {
use payment::poli::*; 
use domain::CurrencyCode;
use serde_json::*;
use domain::Decimal;
use chrono::prelude::*;

#[test]
fn test_request_serializes() {
    let auth_code = AuthenticationCode("9182hrf$902".to_string());
    let poli_tx = PoliTransaction {
        merchant_code: MerchantCode("Best Merchants Ever".to_string()),
        currency_code: CurrencyCode::NZD,
        currency_amount: Decimal::from_cents(30),
        merchant_date_time: Utc::now().naive_utc(),
        successful_url: "https://best-merchants-ever.co.nz/success".to_string(),
        merchant_ref: Some(MerchantRef("best|merch|ever".to_owned())),
        merchant_data: None,
        selected_fi_code: None,
        notification_url: Some("https://best-merchants-ever.co.nz/notification".to_string()),
        unsuccessful_url: None,
        merchant_checkout_url: None,
        timeout: "2000".to_owned(),
        user_ip_address: None
    };
    let d = InitiateTransaction {
        authentication_code: auth_code,
        transaction: poli_tx 
    };
    to_string(&d).unwrap();
}

const REQUEST_EXAMPLE: &'static str = r#"
<?xml version="1.0" encoding="UTF-8"?>
<InitiateTransactionRequest xmlns="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.Contract s" xmlns:i="http://www.w3.org/2001/XMLSchema-instance">
   <AuthenticationCode>MerchantPassword</AuthenticationCode>
   <Transaction xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO">
      <dco:CurrencyAmount>15.00</dco:CurrencyAmount>
      <dco:CurrencyCode>NZD</dco:CurrencyCode>
      <dco:MerchantCheckoutURL>http://www.pricebusterdvd.com/checkout</dco:MerchantCheckoutURL>
      <dco:MerchantCode>PriceBusterDVD</dco:MerchantCode>
      <dco:MerchantData>MerchantDataAssociatedWithTransaction</dco:MerchantData>
      <dco:MerchantDateTime>2008-08-18T14:01:02</dco:MerchantDateTime>
      <dco:MerchantHomePageURL>http://www.pricebusterdvd.com/home</dco:MerchantHomePageURL>
      <dco:MerchantRef>MerchantReferenceAssociateWithTransaction</dco:MerchantRef>
      <dco:NotificationURL>http://www.pricebusterdvd.com/notification</dco:NotificationURL>
      <dco:SelectedFICode i:nil="true" />
      <dco:SuccessfulURL>http://www.pricebusterdvd.com/successful</dco:SuccessfulURL>
      <dco:Timeout>1000</dco:Timeout>
      <dco:UnsuccessfulURL>http://www.pricebusterdvd.com/unsuccessful</dco:UnsuccessfulURL>
      <dco:UserIPAddress>65.2.45.1</dco:UserIPAddress>
   </Transaction>
</InitiateTransactionRequest>
"#;

}

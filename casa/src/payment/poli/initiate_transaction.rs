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

mod test {
use payment::poli::*; 
use domain::CurrencyCode;

#[test]
fn test_request_deserializes() {
    let d: InitiateTransaction = from_str(REQUEST_EXAMPLE.as_bytes()).unwrap();
    assert_eq!("MerchantPassword", d.authentication_code.0);
    assert_eq!("15.00", d.transaction.currency_amount.to_string());
    assert_eq!(CurrencyCode::NZD, d.transaction.currency_code);
    assert_eq!("http://www.pricebusterdvd.com/checkout", d.transaction.merchant_checkout_url.unwrap());
    assert_eq!("PriceBusterDVD", d.transaction.merchant_code.0);
    assert_eq!("MerchantDataAssociatedWithTransaction", d.transaction.merchant_data.unwrap().0);
    assert_eq!("2008-08-18 14:01:02", d.transaction.merchant_date_time.to_string());
    assert_eq!("MerchantReferenceAssociateWithTransaction", d.transaction.merchant_ref.unwrap().0);
    assert_eq!("http://www.pricebusterdvd.com/notification", d.transaction.notification_url.unwrap());
    assert_eq!(Some(String::new()), d.transaction.selected_fi_code);
    assert_eq!("http://www.pricebusterdvd.com/successful", d.transaction.successful_url);
    assert_eq!("1000", d.transaction.timeout);
    assert_eq!("http://www.pricebusterdvd.com/unsuccessful", d.transaction.unsuccessful_url.unwrap());
    assert_eq!("65.2.45.1", d.transaction.user_ip_address.unwrap());
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

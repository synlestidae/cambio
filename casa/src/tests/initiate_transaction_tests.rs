use payment::poli::InitiateTransaction;
use serde_xml_rs::deserialize;

#[test]
fn test_example_deserializes() {
    let d: InitiateTransaction = deserialize(EXAMPLE.as_bytes()).unwrap();
}

const EXAMPLE: &'static str = r#"
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

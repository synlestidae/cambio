use payment::poli::*; //{InitiateTransaction, InitiateTransactionResponse, TransactionStatusCode}
use serde_xml_rs::deserialize;
use domain::CurrencyCode;

#[test]
fn test_request_deserializes() {
    let d: InitiateTransaction = deserialize(REQUEST_EXAMPLE.as_bytes()).unwrap();
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

#[test]
fn test_response_deserializes() {
    let d: InitiateTransactionResponse = 
        deserialize(RESPONSE_EXAMPLE_SUCCESS.as_bytes()).unwrap();
    assert_eq!(d.transaction_status_code, Some(TransactionStatusCode::Initiated));
    let t = d.get_transaction().unwrap();
    assert_eq!(
        "https://txn.apac.paywithpoli.com/?token=%2bXo3AxIuS8T%2fukpoUCZyXw%3d%3d", 
        t.navigate_url
    );
    assert_eq!(
        t.transaction_ref_no.to_string(),
        "996100000001"
    );
    assert_eq!(
        t.transaction_token.to_string(),
        "+Xo3AxIuS8T/ukpoUCZyXw=="
    );
}

#[test]
fn test_response_error_deserializes() {
    let d: InitiateTransactionResponse = 
        deserialize(RESPONSE_EXAMPLE_ERROR.as_bytes()).unwrap();
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

const RESPONSE_EXAMPLE_SUCCESS: &'static str = r#"
<?xml version="1.0" encoding="utf-8"?>
<InitiateTransactionResponse xmlns="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.Contracts" xmlns:i="http://www.w3.org/2001/XMLSchema-instance">
    <Errors xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" />
    <TransactionStatusCode>Initiated</TransactionStatusCode>
    <Transaction xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" >
        <dco:NavigateURL>https://txn.apac.paywithpoli.com/?token=%2bXo3AxIuS8T%2fukpoUCZyXw%3d%3d</dco:NavigateURL>
        <dco:TransactionRefNo>996100000001</dco:TransactionRefNo>
        <dco:TransactionToken>+Xo3AxIuS8T/ukpoUCZyXw==</dco:TransactionToken> 
    </Transaction>
</InitiateTransactionResponse>
"#;

const RESPONSE_EXAMPLE_ERROR: &'static str = r#"
<?xml version="1.0" encoding="utf-8"?>
<InitiateTransactionResponse
xmlns="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.Contract
s" xmlns:i="http://www.w3.org/2001/XMLSchema-instance">
  <Errors
xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO"
>
<dco:Error>
<dco:Code>1003</dco:Code>
<dco:Field />
<dco:Message>POLi is unable to continue with this payment. Please contact the
Merchant for assistance.</dco:Message> </dco:Error>
  </Errors>
  <TransactionStatusCode i:nil="true" />
  <Transaction i:nil="true" xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" />
</InitiateTransactionResponse>
"#;

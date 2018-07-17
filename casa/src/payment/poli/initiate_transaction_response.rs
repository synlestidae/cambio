use payment::poli::{
    InitiateTransactionError, 
    TransactionStatusCode, 
    PoliTransactionResponse
};
use serde::de::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateTransactionResponse {
    #[serde(flatten)]
    pub errors: Option<InitiateTransactionError>,
    #[serde(default)]
    pub transaction_status_code: Option<TransactionStatusCode>,
    #[serde(default)]
    pub transaction: Option<PoliTransactionResponse>
}

impl InitiateTransactionResponse {
    pub fn get_transaction(mut self) 
        -> Result<PoliTransactionResponse, Option<InitiateTransactionError>> {
        match (self.transaction, self.errors) {
            (Some(tx), _) => Ok(tx),
            (_, error) => Err(error)
        }
    }
}

mod test {
    use payment::poli::*; 
    use serde_xml_rs::deserialize;
    use domain::CurrencyCode;

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
    <InitiateTransactionResponse xmlns="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.Contract s" xmlns:i="http://www.w3.org/2001/XMLSchema-instance">
    <Errors xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" >
        <dco:Error>
            <dco:Code>1003</dco:Code>
            <dco:Field />
            <dco:Message>POLi is unable to continue with this payment. Please contact the Merchant for assistance.</dco:Message> 
        </dco:Error>
    </Errors>
    <TransactionStatusCode i:nil="true" />
    <Transaction i:nil="true" xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" />
    </InitiateTransactionResponse>
    "#;

    const RESPONSE_EXAMPLE_ERROR_NOFIELDS: &'static str = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <InitiateTransactionResponse xmlns="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.Contract s" xmlns:i="http://www.w3.org/2001/XMLSchema-instance">
        <Errors xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" >
            <dco:Error>
                <dco:Code>1003</dco:Code>
                <dco:Field />
                <dco:Message>POLi is unable to continue with this payment. Please contact the Merchant for assistance.</dco:Message> 
            </dco:Error>
        </Errors>
        <TransactionStatusCode i:nil="true" />
        <Transaction i:nil="true" xmlns:dco="http://schemas.datacontract.org/2004/07/Centricom.POLi.Services.MerchantAPI.DCO" />
    </InitiateTransactionResponse>
    "#;
}

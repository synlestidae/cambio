use payment::poli::*;
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateTransactionResponse {
    pub success: bool,
    pub transaction_ref_no: Option<TransactionRefNo>,
    #[serde(rename = "NavigateURL")]
    pub navigate_url: Option<String>,
    pub error_code: Option<PoliErrorCode>,
    pub error_message: Option<String>,
}

impl InitiateTransactionResponse {
    pub fn get_transaction(
        mut self,
    ) -> Result<PoliTransactionResponse, Option<InitiateTransactionError>> {
        if self.success {
            match (self.navigate_url, self.transaction_ref_no) {
                (Some(url), Some(reference)) => Ok(PoliTransactionResponse {
                    navigate_url: url,
                    transaction_ref_no: reference,
                }),
                _ => Err(None),
            }
        } else {
            match (self.error_code, self.error_message) {
                (Some(code), Some(msg)) => Err(Some(InitiateTransactionError {
                    error_code: code,
                    error_message: msg,
                })),
                _ => Err(None),
            }
        }
    }
}

mod test {
    use domain::CurrencyCode;
    use payment::poli::*;
    use serde_json::*;

    #[test]
    fn test_response_deserializes() {
        let d: InitiateTransactionResponse = from_str(RESPONSE_EXAMPLE_SUCCESS).unwrap();
        let t = d.get_transaction().unwrap();
        assert_eq!(
            "https://txn.apac.paywithpoli.com/?Token=uo3K8YA7vCojXjA1yuQ3txqX4s26gQSh",
            t.navigate_url
        );
        assert_eq!(t.transaction_ref_no.to_string(), "996117408041");
    }

    #[test]
    fn test_response_error_deserializes() {
        let d: InitiateTransactionResponse = from_str(RESPONSE_EXAMPLE_ERROR_1).unwrap();
        let err = d.get_transaction().err().unwrap().unwrap();
        assert_eq!("5005", err.error_code.0);
        assert_eq!("The certificate was bad", err.error_message);
    }

    const RESPONSE_EXAMPLE_SUCCESS: &'static str = r#"
	{
		"Success": true,
		"NavigateURL": "https://txn.apac.paywithpoli.com/?Token=uo3K8YA7vCojXjA1yuQ3txqX4s26gQSh",
		"ErrorCode": null,
		"ErrorMessage": null,
		"TransactionRefNo": "996117408041"
	}
    "#;

    const RESPONSE_EXAMPLE_ERROR_1: &'static str = r#"
	{
		"Success": false,
		"ErrorCode": "5005",
		"ErrorMessage": "The certificate was bad"
	}
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

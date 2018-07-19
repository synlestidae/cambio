use domain::{CurrencyCode, Decimal};
use payment::poli::*;
use chrono::prelude::*;
//f_x~

#[derive(Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct GetTransactionResponse {
    #[serde(rename="TransactionRefNo", default, with="empty_string_option")]
    pub transaction_ref_no: Option<TransactionRefNo>,
    #[serde(rename="CurrencyCode", default, with="empty_string_option")]
    pub currency_code: Option<CurrencyCode>,
    #[serde(rename="CurrencyName", default, with="empty_string_option")]
    pub currency_name: Option<String>,
    #[serde(rename="CountryCode", default, with="empty_string_option")]
    pub country_code: Option<String>,
    #[serde(rename="CountryName", default, with="empty_string_option")]
    pub country_name: Option<String>,
    #[serde(rename="PaymentAmount", default, with="empty_string_option")]
    pub payment_amount: Option<Decimal>,
    #[serde(rename="AmountPaid", default, with="empty_string_option")]
    pub amount_paid: Option<Decimal>,
    #[serde(with = "poli_date_format_option", rename="EstablishedDateTime")]
    pub established_date_time: Option<NaiveDateTime>,
    #[serde(with = "poli_date_format_option", rename="StartDateTime")]
    pub start_date_time: Option<NaiveDateTime>,
    #[serde(with = "poli_date_format_option", rename="MerchantEstablishedDateTime")]
    pub merchant_established_date_time: Option<NaiveDateTime>,
    #[serde(rename="MerchantReference", default, with="empty_string_option")]
    pub merchant_reference: Option<MerchantRef>,
    #[serde(rename="TransactionStatusCode", default, with="empty_string_option")]
    pub transaction_status_code: Option<TransactionStatusCode>,
    #[serde(rename="BankReceipt", default, with="empty_string_option")]
    pub bank_receipt: Option<String>,
    #[serde(with = "poli_date_format_option", rename="BankReceiptDateTime", default)]
    pub bank_receipt_date_time: Option<NaiveDateTime>,
    #[serde(rename="ErrorCode", default, with="empty_string_option")]
    pub error_code: Option<PoliErrorCode>,
    #[serde(rename="ErrorMessage", default, with="empty_string_option")]
    pub error_message: Option<String>,
    #[serde(rename="FinancialInstitutionCountryCode", default, with="empty_string_option")]
    pub financial_institution_country_code: Option<String>,
    #[serde(rename="FinancialInstitutionCode", default, with="empty_string_option")]
    pub financial_institution_code: Option<String>,
    #[serde(rename="FinancialInstitutionName", default, with="empty_string_option")]
    pub financial_institution_name: Option<String>,
    #[serde(rename="MerchantData", default, with="empty_string_option")]
    pub merchant_defined_data: Option<MerchantData>,
    #[serde(rename="MerchantAccountName", default, with="empty_string_option")]
    pub merchant_acct_name: Option<String>,
    #[serde(rename="MerchantSortCode", default, with="empty_string_option")]
    pub merchant_acct_sort_code: Option<String>,
    #[serde(rename="MerchantAccountSuffix", default, with="empty_string_option")]
    pub merchant_acct_suffix: Option<String>,
    #[serde(rename="MerchantAccountNumber", default, with="empty_string_option")]
    pub merchant_acct_number: Option<String>,
}

impl GetTransactionResponse {
    pub fn get_transaction(&self) -> Result<RemoteTransaction, Option<RemoteTransactionError>> {
        match (&self.transaction_ref_no, &self.currency_code, &self.payment_amount, &self.amount_paid, &self.transaction_status_code) {
            (Some(ref tx_ref), Some(ref code), Some(ref amount), Some(ref paid), Some(ref status)) => {
                return Ok(RemoteTransaction {
                    transaction_ref_no: tx_ref.clone(),
                    currency_code: code.clone(),
                    payment_amount: amount.clone(),
                    amount_paid: paid.clone(),
                    transaction_status_code: status.clone()
                });
            },
            _ => ()
        };

        Err(if let Some(error_code) = self.error_code {
            Some(RemoteTransactionError {
                error_code: error_code,
                error_message: self.error_message.clone()
            })
        } else {
            None
        })
    }
}

mod test {
    use serde_json::from_str;
    use payment::poli::*;

    #[test]
    fn test_get_transaction_response_deserializes() {
        let tx_response: GetTransactionResponse = from_str(EXAMPLE).unwrap();
        assert_eq!(tx_response.transaction_ref_no.unwrap().to_string(), "996108109898");
        assert_eq!(tx_response.currency_code.unwrap().to_string(), "AUD");
        assert_eq!(tx_response.currency_name.unwrap(), "Australian Dollar");
        assert_eq!(tx_response.country_code.unwrap(), "AU");
        assert_eq!(tx_response.country_name.unwrap(), "Australia");
        assert_eq!(tx_response.payment_amount.unwrap().to_string(), "1.27");
        assert_eq!(tx_response.amount_paid.unwrap().to_string(), "0.00");
        assert_eq!(tx_response.established_date_time.unwrap().to_string(), "2018-02-27 15:19:55.063");
        assert_eq!(tx_response.start_date_time.unwrap().to_string(), "2018-02-27 15:19:55.063");
        assert_eq!(tx_response.merchant_established_date_time.unwrap().to_string(), "2018-02-27 15:19:54.123");
        assert_eq!(tx_response.merchant_reference.unwrap().0, "MyRef01");
        assert_eq!(tx_response.transaction_status_code, Some(TransactionStatusCode::EulaAccepted));
        assert_eq!(tx_response.bank_receipt, None);
        assert_eq!(tx_response.bank_receipt_date_time, None);
        assert_eq!(tx_response.error_code, None);
        assert_eq!(tx_response.error_message, None);
        assert_eq!(tx_response.financial_institution_country_code, None);
        assert_eq!(tx_response.financial_institution_code, None);
        assert_eq!(tx_response.financial_institution_name, None);
        assert_eq!(tx_response.merchant_defined_data.unwrap().0, "MyDefinedData");
        assert_eq!(tx_response.merchant_acct_name.unwrap(), "TEST");
        assert_eq!(tx_response.merchant_acct_sort_code, None); 
        assert_eq!(tx_response.merchant_acct_suffix, None);
        assert_eq!(tx_response.merchant_acct_number.unwrap(), "35313843");
    }

    #[test]
    fn test_get_transaction_response_with_nulls_deserializes() {
        let tx_response: GetTransactionResponse = from_str(EXAMPLE_NULL).unwrap();
    }

    const EXAMPLE: &'static str = r#"{
        "CountryName": "Australia",
        "FinancialInstitutionCountryCode": "",
        "TransactionID": "9c9955a7-53d1-4a33-b076-c53435e00225",
        "MerchantEstablishedDateTime": "2018-02-27T15:19:54.123",
        "CurrencyCode": "AUD",
        "PayerAccountNumber": "",
        "PayerAccountSortCode": "",
        "MerchantAccountSortCode": "923100",
        "MerchantAccountName": "TEST",
        "MerchantData": "MyDefinedData",
        "CurrencyName": "Australian Dollar",
        "TransactionStatus": "EulaAccepted",
        "IsExpired": false,
        "MerchantEntityID": "e04dc5a4-8cf9-4af0-98df-669f3bb05aab",
        "UserIPAddress": "127.0.0.1",
        "POLiVersionCode": "  ",
        "MerchantName": "Pricebuster AU",
        "TransactionRefNo": "996108109898",
        "CountryCode": "AU",
        "PaymentAmount": 1.27,
        "AmountPaid": 0,
        "EstablishedDateTime": "2018-02-27T15:19:55.063",
        "StartDateTime": "2018-02-27T15:19:55.063",
        "EndDateTime": null,
        "BankReceipt": "",
        "BankReceiptDateTime": "",
        "TransactionStatusCode": "EulaAccepted",
        "ErrorCode": null,
        "ErrorMessage": "",
        "FinancialInstitutionCode": "",
        "FinancialInstitutionName": "",
        "MerchantReference": "MyRef01",
        "MerchantAccountSuffix": null,
        "MerchantAccountNumber": "35313843",
        "PayerFirstName": "",
        "PayerFamilyName": "",
        "PayerAccountSuffix": ""
    }"#;

    const EXAMPLE_NULL: &'static str = r#"{
        "CountryName": "Australia",
        "TransactionID": "9c9955a7-53d1-4a33-b076-c53435e00225",
        "MerchantEstablishedDateTime": "2018-02-27T15:19:54.123",
        "CurrencyCode": "AUD",
        "MerchantAccountSortCode": "923100",
        "MerchantAccountName": "TEST",
        "MerchantData": "MyDefinedData",
        "CurrencyName": "Australian Dollar",
        "TransactionStatus": "EulaAccepted",
        "IsExpired": false,
        "MerchantEntityID": "e04dc5a4-8cf9-4af0-98df-669f3bb05aab",
        "MerchantName": "Pricebuster AU",
        "TransactionRefNo": "996108109898",
        "CountryCode": "AU",
        "PaymentAmount": 1.27,
        "AmountPaid": 0,
        "EstablishedDateTime": "2018-02-27T15:19:55.063",
        "StartDateTime": "2018-02-27T15:19:55.063",
        "TransactionStatusCode": "EulaAccepted",
        "MerchantReference": "MyRef01",
        "MerchantAccountNumber": "35313843"
    }"#;

}

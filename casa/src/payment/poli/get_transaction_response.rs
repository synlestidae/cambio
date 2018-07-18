use domain::{CurrencyCode, Decimal};
use payment::poli::*;
use chrono::prelude::*;
//f_x~

#[derive(Deserialize)]
#[serde(rename_all="PascalCase")]
pub struct GetTransactionResponse {
    #[serde(rename="TransactionRefNo")]
    pub transaction_ref_no: TransactionRefNo,
    #[serde(rename="CurrencyCode")]
    pub currency_code: CurrencyCode,
    #[serde(rename="CurrencyName")]
    pub currency_name: String,
    #[serde(rename="CountryCode")]
    pub country_code: String,
    #[serde(rename="CountryName")]
    pub country_name: String,
    #[serde(rename="PaymentAmount")]
    pub payment_amount: Decimal,
    #[serde(rename="AmountPaid")]
    pub amount_paid: Decimal,
    #[serde(with = "poli_date_format", rename="EstablishedDateTime")]
    pub established_date_time: NaiveDateTime,
    #[serde(with = "poli_date_format", rename="StartDateTime")]
    pub start_date_time: NaiveDateTime,
    #[serde(rename="BankReceipt")]
    pub bank_receipt: Option<String>,
    #[serde(with = "poli_date_format_option", rename="BankReceiptDateTime")]
    pub bank_receipt_date_time: Option<NaiveDateTime>,
    #[serde(rename="TransactionStatusCode")]
    pub transaction_status_code: TransactionStatusCode,
    #[serde(rename="ErrorCode")]
    pub error_code: Option<PoliErrorCode>,
    #[serde(rename="ErrorMessage")]
    pub error_message: Option<String>,
    #[serde(rename="FinancialInstitutionCountryCode")]
    pub financial_institution_country_code: String,
    #[serde(rename="FinancialInstitutionCode")]
    pub financial_institution_code: String,
    #[serde(rename="FinancialInstitutionName")]
    pub financial_institution_name: String,
    #[serde(with = "poli_date_format", rename="MerchantEstablishedDateTime")]
    pub merchant_established_date_time: NaiveDateTime,
    #[serde(rename="MerchantReference")]
    pub merchant_reference: MerchantRef,
    #[serde(rename="MerchantDefinedData")]
    pub merchant_defined_data: Option<MerchantData>,
    #[serde(rename="MerchantAccountName")]
    pub merchant_acct_name: Option<String>,
    #[serde(rename="MerchantSortCode")]
    pub merchant_acct_sort_code: Option<String>,
    #[serde(rename="MerchantAccountSuffix")]
    pub merchant_acct_suffix: Option<String>,
    #[serde(rename="MerchantAccountNumber")]
    pub merchant_acct_number: Option<String>,
}

mod test {
    use serde_json::from_str;
    use payment::poli::*;

    #[test]
    fn test_get_transaction_response_deserializes() {
        let tx_response: GetTransactionResponse = from_str(EXAMPLE).unwrap();
    }

    const EXAMPLE: &'static str = r#"
{
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

}

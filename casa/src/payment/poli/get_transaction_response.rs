use domain::{CurrencyCode, Decimal};
use payment::poli::*;
use chrono::prelude::*;
//f_x~

#[derive(Serialize, Deserialize)]
pub struct GetTransactionResponse {
    #[serde(rename="TransactionRefNo")]
    pub transaction_ref_no: TransactionRefNo,
    #[serde(rename="CurrencyCode")]
    pub currency_code: CurrencyCode,
    #[serde(rename="CurrencyName")]
    pub currency_name: String,
    #[serde(rename="CurrencyCode")]
    pub country_code: String,
    #[serde(rename="CurrencyName")]
    pub country_name: String,
    #[serde(rename="PaymentAmount")]
    pub payment_amount: Decimal,
    #[serde(rename="AmountPaid")]
    pub amount_paid: Decimal,
    #[serde(rename="EstablishedDateTime")]
    pub established_date_time: NaiveDate,
    #[serde(rename="StartDateTime")]
    pub start_date_time: NaiveDate,
    #[serde(rename="BankReceipt")]
    pub bank_receipt: Option<String>,
    #[serde(rename="BankReceiptDateTime")]
    pub bank_receipt_date_time: Option<NaiveDate>,
    #[serde(rename="TransactionStatusCode")]
    pub transaction_status_code: TransactionStatusCode,
    #[serde(rename="ErrorCode")]
    pub error_code: Option<String>,
    #[serde(rename="ErrorMessage")]
    pub error_message: Option<String>,
    #[serde(rename="FinancialInstitutionCountryCode")]
    pub financial_institution_country_code: String,
    #[serde(rename="FinancialInstitutionCode")]
    pub financial_institution_code: String,
    #[serde(rename="FinancialInstitutionName")]
    pub financial_institution_name: String,
    #[serde(rename="MerchantEstablishedDateTime")]
    pub merchant_established_date_time: NaiveDate,
    #[serde(rename="MerchantReference")]
    pub merchant_reference: MerchantRef,
    #[serde(rename="MerchantDefinedData")]
    pub merchant_defined_data: MerchantData,
    #[serde(rename="MerchantAcctName")]
    pub merchant_acct_name: String,
    #[serde(rename="MerchantSortCode")]
    pub merchant_acct_sort_code: String,
    #[serde(rename="MerchantAcctSuffix")]
    pub merchant_acct_suffix: String,
    #[serde(rename="MerchantAcctNumber")]
    pub merchant_acct_number: String,
}

use domain::{CurrencyCode, Decimal};
use payment::poli::*;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct GetTransactionResponse {
    pub transaction_ref_no: TransactionRefNo,
    pub currency_code: CurrencyCode,
    pub currency_name: String,
    pub country_code: String,
    pub country_name: String,
    pub payment_amount: Decimal,
    pub amount_paid: Decimal,
    pub established_date_time: NaiveDate,
    pub start_date_time: NaiveDate,
    pub bank_receipt: Option<String>,
    pub bank_receipt_date_time: Option<NaiveDate>,
    pub transaction_status_code: TransactionStatusCode,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub financial_institution_country_code: String,
    pub financial_institution_code: String,
    pub financial_institution_name: String,
    pub merchant_established_date_time: NaiveDate,
    pub merchant_reference: MerchantRef,
    pub merchant_defined_data: MerchantData,
    pub merchant_acct_name: String,
    pub merchant_acct_sort_code: String,
    pub merchant_acct_suffix: String,
    pub merchant_acct_number: String,
}

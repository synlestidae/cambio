use chrono::prelude::*;
use domain::{CurrencyCode, Decimal};
use payment::poli::*;

#[derive(Debug)]
pub struct RemoteTransaction {
    pub transaction_ref_no: TransactionRefNo,
    pub currency_code: CurrencyCode,
    pub payment_amount: Decimal,
    pub amount_paid: Decimal,
    pub transaction_status_code: TransactionStatusCode,
}

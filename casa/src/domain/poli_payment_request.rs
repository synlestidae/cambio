use domain::{PoliPaymentRequestId, PaymentStatus, Code, UserId, Decimal};
use chrono::prelude::*;
use payment::poli::TransactionRefNo;
use postgres;
use db::{TryFromRow, TryFromRowError};

#[derive(FromSql, ToSql, Debug, TryFromRow)]
pub struct PoliPaymentRequest {
    pub id: Option<PoliPaymentRequestId>, 
    pub user_id: UserId,
    pub amount: Decimal,
    pub unique_code: Code,
    pub started_at: DateTime<Utc>, 
    pub payment_status: PaymentStatus, 
    pub transaction_ref_no: Option<TransactionRefNo>,
}

impl PoliPaymentRequest {
    pub fn new(user_id: UserId, amount: Decimal) -> Self {
        PoliPaymentRequest {
            id: None,
            user_id: user_id,
            amount: amount,
            unique_code: Code::new(),
            started_at: Utc::now(),
            payment_status: PaymentStatus::StartedByUser,
            transaction_ref_no: None
        }
    }
}

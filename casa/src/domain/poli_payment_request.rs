use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use domain::{Code, Decimal, PaymentStatus, PoliPaymentRequestId, UserId};
use payment::poli::TransactionRefNo;
use postgres;

#[derive(FromSql, ToSql, Debug, TryFromRow)]
pub struct PoliPaymentRequest {
    pub id: Option<PoliPaymentRequestId>,
    pub user_id: UserId,
    pub amount: Decimal,
    pub unique_code: Code,
    pub started_at: DateTime<Utc>,
    pub payment_status: PaymentStatus,
    pub transaction_ref_no: Option<TransactionRefNo>,
    #[column_name(amount_paid_cents)]
    pub amount_paid: Decimal,
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
            transaction_ref_no: None,
            amount_paid: Decimal::from_dollars(0),
        }
    }
}

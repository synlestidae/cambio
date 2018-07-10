use domain::{PoliPaymentRequestId, PaymentStatus, Code, UserId, Decimal};
use chrono::prelude::*;
use payment::poli::TransactionRefNo;

pub struct PoliPaymentRequest {
    pub id: Option<PoliPaymentRequestId>, 
    pub unique_code: Code,
    pub amount: Decimal,
    pub user_id: UserId,
    pub started_at: DateTime<Utc>, 
    pub payment_status: PaymentStatus, 
    pub transaction_ref_no: Option<TransactionRefNo>
}

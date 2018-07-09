use domain::{PoliPaymentRequestId, PaymentStatus, Code, UserId, Decimal};
use chrono::prelude::*;

pub struct PoliPaymentRequest {
    id: Option<PoliPaymentRequestId>, 
    unique_code: Code,
    amount: Decimal,
    user_id: UserId,
    started_at: DateTime<Utc>, 
    payment_status: PaymentStatus, 
    transaction_ref_no: Code
}

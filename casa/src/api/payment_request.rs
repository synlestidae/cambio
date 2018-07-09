use domain::{Decimal, UniqueId, CurrencyCode, PaymentMethod}; //Payment;

#[derive(Debug, Deserialize, Clone)]
pub struct PaymentRequest {
    amount: Decimal,
    unique_id: UniqueId,
    currency_code: CurrencyCode,
    method: PaymentMethod
}

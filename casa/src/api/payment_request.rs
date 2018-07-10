use domain::{Decimal, UniqueId, CurrencyCode, PaymentMethod}; //Payment;

#[derive(Debug, Deserialize, Clone)]
pub struct PaymentRequest {
    pub amount: Decimal,
    pub unique_id: UniqueId,
    pub currency_code: CurrencyCode,
    pub method: PaymentMethod
}

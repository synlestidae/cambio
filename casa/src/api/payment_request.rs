use domain::{CurrencyCode, Decimal, PaymentMethod, UniqueId}; //Payment;

#[derive(Debug, Deserialize, Clone)]
pub struct PaymentRequest {
    pub amount: Decimal,
    pub unique_id: UniqueId,
    pub currency_code: CurrencyCode,
    pub method: PaymentMethod,
}

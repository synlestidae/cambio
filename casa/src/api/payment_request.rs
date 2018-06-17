use domain::Payment;

#[derive(Debug)]
pub enum PaymentRequest {
    CreditCardPayment(Payment)
}

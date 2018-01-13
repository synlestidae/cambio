#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "payment_method")]
pub enum PaymentMethod {
    #[postgres(name = "nz_bank_deposit")]
    NZBankDeposit,
    #[postgres(name = "credit_card")]
    CreditCard,
}

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "account_type")]
pub enum AccountType {
    #[postgres(name = "credit_normal")]
    CreditNormal,
    #[postgres(name = "debit_normal")]
    DebitNormal
}

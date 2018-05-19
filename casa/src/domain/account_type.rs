#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "account_type")]
pub enum AccountType {
    #[postgres(name = "asset")] Asset,
    #[postgres(name = "liability")] Liability,
    #[postgres(name = "equity")] Equity,
    #[postgres(name = "income")] Income,
    #[postgres(name = "expense")] Expense,
}

use domain::Account;
use api::TransactionInfo;

#[derive(Debug, Clone, Serialize)]
pub struct AccountStatement {
    pub account: Account,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub transactions: Vec<TransactionInfo>,
}

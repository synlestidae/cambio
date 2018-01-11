use domain::{Account, Transaction};

#[derive(Debug, Clone)]
pub struct AccountStatement {
    pub account: Account,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub transactions: Vec<Transaction>,
}

use domain::{Account, Transaction};

pub struct AccountStatement {
    pub account: Account,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub transactions: Vec<Transaction>
}

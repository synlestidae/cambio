use domain;

#[derive(Debug)]
pub enum AccountRequest {
    GetAccounts,
    GetAccount(domain::AccountId),
    GetAccountTransactions(domain::AccountId),
}

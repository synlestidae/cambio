use api;
use domain::{Id, Account, Transaction};
use api::{ApiResult, ApiError};

pub trait AccountApiTrait {
    fn get_accounts(&mut self) -> ApiResult<Vec<Account>>;
    fn get_account(&mut self, account_id: &Id) -> ApiResult<Account>;
    fn get_transactions(&mut self, account_id: &Id) -> ApiResult<Vec<Transaction>>;
    fn get_transaction(&mut self, account_id: &Id, transaction_id: &Id) -> ApiResult<Transaction>;
}
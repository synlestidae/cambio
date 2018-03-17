use api;
use domain::{Id, Account, Transaction};
use api::{ApiResult, ApiError};
use iron;

pub trait AccountApiTrait {
    fn get_accounts(&mut self, request: &iron::Request) -> iron::Response;
    fn get_account(&mut self, account_id: Id, session_token: &str) -> ApiResult<Account>;
    fn get_transactions(&mut self, account_id: Id, session_token: &str) -> ApiResult<Vec<Transaction>>;
    fn get_transaction(&mut self, account_id: Id, transaction_id: Id, session_token: &str) -> ApiResult<Transaction>;
}

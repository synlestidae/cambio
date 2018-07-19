use api;
use api::{ApiError, ApiResult};
use domain::{Account, AccountId, TransactionId, User};
use iron;

pub trait AccountApiTrait {
    fn get_accounts(&mut self, user: &User) -> iron::Response;
    fn get_account(&mut self, user: &User, account_id: AccountId) -> iron::Response;
    fn get_transactions(&mut self, user: &User, account_id: AccountId) -> iron::Response;
    fn get_transaction(
        &mut self,
        user: &User,
        account_id: AccountId,
        tx_id: TransactionId,
    ) -> iron::Response;
}

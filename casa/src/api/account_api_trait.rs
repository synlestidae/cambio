use api;
use domain::{Id, Account, Transaction};
use api::{ApiResult, ApiError};
use iron;

pub trait AccountApiTrait {
    fn get_accounts(&mut self, request: &iron::Request) -> iron::Response;
    fn get_account(&mut self, request: &iron::Request) -> iron::Response;
    fn get_transactions(&mut self, request: &iron::Request) -> iron::Response;
    fn get_transaction(&mut self, request: &iron::Request) -> iron::Response;
}

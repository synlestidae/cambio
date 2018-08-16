use api::utils::{get_session_token, to_response};
use api::{ApiError, ApiResult, ErrorType, TransactionInfo};
use db::{CambioError, ConnectionSource, PostgresHelper};
use domain::*;
use hyper::mime::Mime;
use iron;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::prelude::*;
use iron::request::Request;
use params::{Params, Value};
use postgres::GenericConnection;
use repository::Readable;
use serde::Serialize;
use serde_json;
use services::AccountService;

pub struct AccountApiImpl<C: GenericConnection> {
    db: C,
    account_service: AccountService,
}

impl<C: GenericConnection> AccountApiImpl<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db,
            account_service: AccountService::new(),
        }
    }

    pub fn get_statement(
        &mut self,
        user: &User,
        account_id: AccountId,
    ) -> Result<AccountStatement, CambioError> {
        let account_service = AccountService::new();
        let account: Account = account_id.get(&mut self.db)?;;
        if user.owner_id != account.owner_user_id {
            return Err(CambioError::not_found_search("Account not found", "Could not find that account"));
        }
        self.account_service
            .get_latest_statement(&mut self.db, account_id)
    }

    pub fn get_accounts(&mut self, user: &User) -> iron::Response {
        let accounts: Vec<Account> = match user.owner_id.unwrap().get_vec(&mut self.db) {
            Ok(a) => a,
            Err(err) => {
                let api_error: ApiError = err.into();
                return api_error.into();
            }
        };
        let visible_accounts = accounts
            .into_iter()
            .filter(|a| a.is_user_visible())
            .collect();
        to_response::<Vec<Account>>(Ok(visible_accounts))
    }

    pub fn get_account(&mut self, user: &User, account_id: AccountId) -> iron::Response {
        let account: Account = match account_id.get(&mut self.db) {
            Ok(account) => account,
            Err(err) => return err.into(),
        };
        to_response(Ok(account))
    }

    pub fn get_transactions(&mut self, user: &User, account_id: AccountId) -> iron::Response {
        match self.get_statement(user, account_id) {
            Ok(statement) => to_response(Ok(statement.transactions)),
            err => to_response(err)
        }
    }
}

pub fn get_param(request: &Request, idx: usize) -> Option<Id> {
    let mut url = request.url.path(); //.clone();
    let id = match url[idx] {
        id_string => Id(i32::from_str_radix(id_string, 10).unwrap()),
    };
    Some(id)
}

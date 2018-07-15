use api::utils::{get_session_token, to_response};
use api::{AccountApiTrait, ApiError, ApiResult, ErrorType};
use db::{CambioError, ConnectionSource, PostgresHelper};
use domain::{Account, AccountStatement, Id, OwnerId, Session, Transaction, User, AccountId, TransactionId};
use hyper::mime::Mime;
use iron;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::prelude::*;
use iron::request::Request;
use params::{Params, Value};
use repository::{Readable};
use serde::Serialize;
use serde_json;
use postgres::GenericConnection;

#[derive(Clone)]
pub struct AccountApiImpl<C: GenericConnection> {
    db: C
}

impl<C: GenericConnection> AccountApiImpl<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db
        }
    }

    fn get_statement(&mut self, user: &User, account_id: AccountId) -> Result<AccountStatement, iron::Response> {
        let account = match account_id.get(&mut self.db) {
            Ok(a) => a,
            Err(err) => return Err(err.into())
        };
        if user.owner_id != account.owner_user_id {
            return Err(ApiError::not_found("Account").into());
        }
        match self.account_service.get_latest_statement(account_id) {
            Ok(s) => Ok(s),
            err => Err(to_response(err)),
        }
    }
}

impl<C: GenericConnection> AccountApiTrait for AccountApiImpl<C> {
    fn get_accounts(&mut self, user: &User) -> iron::Response {
        let accounts = match user.owner_id.get_vec(&mut self.db) {
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

    fn get_account(&mut self, user: &User, account_id: AccountId) -> iron::Response {
        match account_id.get(&mut self.db) {
            Ok(account) => to_response(Ok(account)),
            Err(err) => err.into(),
        }
    }

    fn get_transactions(&mut self, user: &User, account_id: AccountId) -> iron::Response {
        match self.get_statement(user, account_id) {
            Ok(statement) => to_response(Ok(statement.transactions)),
            Err(err) => err,
        }
    }

    fn get_transaction(&mut self, user: &User, account_id: AccountId, tx_id: TransactionId) -> iron::Response {
        let statement = match self.get_statement(user, account_id) {
            Ok(s) => s,
            Err(err) => return err,
        };
        for tx in statement.transactions.into_iter() {
            if tx.id == tx_id {
                return to_response(Ok(tx));
            }
        }
        to_response::<Transaction>(Err(CambioError::not_found_search(
            &format!("Transaction with ID {:?} not found in latest statement", tx_id),
            "Could not find that transaction",
        )))
    }
}

pub fn get_param(request: &Request, idx: usize) -> Option<Id> {
    let mut url = request.url.path(); //.clone();
    let id = match url[idx] {
        id_string => Id(i32::from_str_radix(id_string, 10).unwrap()),
    };
    Some(id)
}

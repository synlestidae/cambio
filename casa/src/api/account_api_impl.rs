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
use repositories::{AccountRepository, SessionRepository, UserRepository};
use repository;
use repository::RepoRead;
use serde::Serialize;
use serde_json;
use services::AccountService;
use repository::Readable;

#[derive(Clone)]
pub struct AccountApiImpl<C: PostgresHelper + Clone> {
    account_repo: AccountRepository<C>,
    account_service: AccountService<C>,
    session_repo: SessionRepository<C>,
    user_repo: UserRepository<C>,
    db: C,
}

impl<C: PostgresHelper + Clone> AccountApiImpl<C> {
    pub fn new(helper: C) -> Self {
        Self {
            account_repo: AccountRepository::new(helper.clone()),
            account_service: AccountService::new(helper.clone()),
            session_repo: SessionRepository::new(helper.clone()),
            user_repo: UserRepository::new(helper.clone()),
            db: helper
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
        match self.account_service.get_latest_statement(Id(account_id.0)) {
            Ok(s) => Ok(s),
            err => Err(to_response(err)),
        }
    }

    fn check_owner(&mut self, owner_id: OwnerId, session_token: &str) -> Result<(), ApiError> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = self.session_repo.read(&clause).unwrap().pop().unwrap();
        if !session.is_valid() {
            return Err(ApiError::new(
                "You are not logged in.".to_owned(),
                ErrorType::NotLoggedIn,
            ));
        }
        let user = self.user_repo.get_owner(owner_id).unwrap();
        if session.email_address.unwrap() != user.email_address {
            return Err(ApiError::new(
                "Cannot retrieve object.".to_owned(),
                ErrorType::NotFound,
            ));
        }
        Ok(())
    }

    fn get_session(&mut self, request: &Request) -> Result<Session, iron::Response> {
        let session_token_match = get_session_token(request);
        let session_token = match session_token_match {
            Some(t) => t,
            None => return Err(ApiError::unauthorised().into()),
        };

        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        match self.session_repo.read(&clause).map(|mut s| s.pop()) {
            Ok(Some(session)) => Ok(session),
            _ => Err(ApiError::unauthorised().into()),
        }
    }

    fn _get_account(&mut self, request: &Request) -> Option<Account> {
        let account_id = get_param(request, 1).unwrap();
        let clause = repository::UserClause::Id(account_id);
        let session_token = get_session_token(request).unwrap();
        let session_clause = repository::UserClause::SessionToken(session_token);
        let mut accounts = self.account_repo.read(&clause).unwrap();
        let session = self
            .session_repo
            .read(&session_clause)
            .unwrap()
            .pop()
            .unwrap();
        let account = accounts.pop();
        if let &Some(ref a) = &account {
            let owner_user_id = a.owner_user_id.unwrap();
            let user_clause = repository::UserClause::Id(owner_user_id.into());
            let account_user = self.user_repo.read(&user_clause).unwrap().pop().unwrap();
            if session.email_address != Some(account_user.email_address) {
                return None;
            }
        }
        account
    }
}

impl<C: PostgresHelper + Clone> AccountApiTrait for AccountApiImpl<C> {
    fn get_accounts(&mut self, user: &User) -> iron::Response {
        let email_clause = repository::UserClause::EmailAddress(user.email_address.clone());
        let accounts = match self.account_repo.read(&email_clause) {
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

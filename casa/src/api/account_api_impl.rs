use api::{AccountApiTrait, ApiError, ApiResult, ErrorType};
use db::{CambioError, ConnectionSource, PostgresHelper};
use domain::{Account, AccountStatement, Id, Session, Transaction};
use hyper::mime::Mime;
use iron::headers::{Authorization, Bearer, Cookie};
use api::utils::{get_session_token, to_response};
use iron::request::Request;
use iron;
use iron::prelude::*;
use repositories::{AccountRepository, SessionRepository, UserRepository};
use repository::RepoRead;
use repository;
use serde_json;
use serde::Serialize;
use services::AccountService;
use params::{Params, Value};

#[derive(Clone)]
pub struct AccountApiImpl<C: PostgresHelper> {
    account_repo: AccountRepository<C>,
    account_service: AccountService<C>,
    session_repo: SessionRepository<C>,
    user_repo: UserRepository<C>,
}

impl<C: PostgresHelper> AccountApiImpl<C> {
    pub fn new(helper: C) -> Self {
        Self {
            account_repo: AccountRepository::new(helper.clone()),
            account_service: AccountService::new(helper.clone()),
            session_repo: SessionRepository::new(helper.clone()),
            user_repo: UserRepository::new(helper),
        }
    }

    fn get_statement(&mut self, request: &Request) -> Result<AccountStatement, iron::Response> {
        let account_id = get_param(request, 1).unwrap();
        let session_token = get_session_token(request).unwrap();
        let account = self._get_account(request).unwrap();
        if let Err(err) = self.check_owner(account.owner_user_id.unwrap(), &session_token) {
            return Err(err.into());
        }
        match self.account_service.get_latest_statement(account_id) {
            Ok(s) => Ok(s),
            err => Err(to_response(err)),
        }
    }

    fn check_owner(&mut self, owner_id: Id, session_token: &str) -> Result<(), ApiError> {
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
        let session = self.session_repo
            .read(&session_clause)
            .unwrap()
            .pop()
            .unwrap();
        let account = accounts.pop();
        if let &Some(ref a) = &account {
            let owner_user_id = a.owner_user_id.unwrap();
            let user_clause = repository::UserClause::Id(owner_user_id);
            let account_user = self.user_repo.read(&user_clause).unwrap().pop().unwrap();
            if session.email_address != Some(account_user.email_address) {
                return None;
            }
        }
        account
    }
}

impl<C: PostgresHelper> AccountApiTrait for AccountApiImpl<C> {
    fn get_accounts(&mut self, request: &Request) -> iron::Response {
        let session = match self.get_session(request) {
            Ok(s) => s,
            Err(_) => return ApiError::unauthorised().into(),
        };
        let email_clause = repository::UserClause::EmailAddress(session.email_address.unwrap());
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

    fn get_account(&mut self, request: &Request) -> iron::Response {
        let account = self._get_account(request);
        match account {
            Some(account) => to_response(Ok(account)),
            None => ApiError::not_found("Account").into(),
        }
    }

    fn get_transactions(&mut self, request: &Request) -> iron::Response {
        match self.get_statement(request) {
            Ok(statement) => to_response(Ok(statement.transactions)),
            Err(err) => err,
        }
    }

    fn get_transaction(&mut self, request: &Request) -> iron::Response {
        let statement = match self.get_statement(request) {
            Ok(s) => s,
            Err(err) => return err,
        };
        let id = match get_param(request, 1) {
            Some(id) => id,
            None => {
                return to_response::<Transaction>(Err(CambioError::format_obj(
                    "Cannot parse ID from URL",
                    "Could not get param at index 1",
                )))
            }
        };
        for tx in statement.transactions.into_iter() {
            if tx.id == id {
                return to_response(Ok(tx));
            }
        }
        to_response::<Transaction>(Err(CambioError::not_found_search(
            &format!("Transaction with ID {} not found in latest statement", id),
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

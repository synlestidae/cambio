use api::{AccountApiTrait, ApiResult, ApiError, ErrorType};
use db::{ConnectionSource, PostgresHelper, CambioError};
use domain::{Account, Id, Transaction, Session};
use hyper::mime::{Mime};
use iron::headers::{Cookie, Authorization, Bearer};
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
    user_repo: UserRepository<C>
}

impl<C: PostgresHelper> AccountApiImpl<C> {
    pub fn new(helper: C) -> Self {
        Self {
            account_repo: AccountRepository::new(helper.clone()),
            account_service: AccountService::new(helper.clone()),
            session_repo: SessionRepository::new(helper.clone()),
            user_repo: UserRepository::new(helper)
        }
    }

    fn check_owner(&mut self, owner_id: Id, session_token: &str) -> Result<(), ApiError> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = self.session_repo.read(&clause).unwrap().pop().unwrap();
        if !session.is_valid() {
            return Err(ApiError::new("You are not logged in.".to_owned(), ErrorType::NotLoggedIn));
        }
        let user = self.user_repo.get_owner(owner_id).unwrap();
        if session.email_address.unwrap() != user.email_address {
            return Err(ApiError::new("Cannot retrieve object.".to_owned(), ErrorType::NotFound));
        }
        Ok(())
    }

    fn get_session(&mut self, request: &Request) -> Result<Session, iron::Response> {
        let session_token_match = get_session_token(request);
        let session_token = match session_token_match {
            Some(t) => t,
            None => return Err(ApiError::unauthorised().into())
        };

        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        match self.session_repo.read(&clause).map(|mut s| s.pop())  {
            Ok(Some(session)) => Ok(session),
            _ => Err(ApiError::unauthorised().into())
        }
    }

    fn _get_account(&mut self, request: &Request) -> Option<Account> {
        let account_id = get_account_id(request).unwrap();
        let clause = repository::UserClause::Id(account_id);
        let session_token = get_session_token(request).unwrap();
        let session_clause = repository::UserClause::SessionToken(session_token);
        let mut accounts = self.account_repo.read(&clause).unwrap();
        let session = self.session_repo.read(&session_clause).unwrap().pop().unwrap();
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
            Err(_) => return ApiError::unauthorised().into()
        };
        let email_clause = repository::UserClause::EmailAddress(session.email_address.unwrap());
        let accounts = match self.account_repo.read(&email_clause) {
            Ok(a) => a,
            Err(err) => {
                let api_error: ApiError = err.into();
                return api_error.into();
            }
        };
        let visible_accounts = accounts.into_iter().filter(|a| a.is_user_visible()).collect();
        to_response::<Vec<Account>>(Ok(visible_accounts))
    }

    fn get_account(&mut self, request: &Request) 
        -> iron::Response { 
        let account = self._get_account(request);
        match account {
            Some(account) => {
                to_response(Ok(account))
            }, 
            None => {
                ApiError::not_found("Account").into()
            }
        }
    }

    fn get_transactions(&mut self, request: &Request) -> iron::Response {
        let account_id = get_account_id(request).unwrap();
        let session_token = get_session_token(request).unwrap();
        println!("Token black {:?}", session_token);
        let account = self._get_account(request).unwrap();
        if let Err(err) = self.check_owner(account.owner_user_id.unwrap(), &session_token) {
            println!("Owner check failed!");
            return err.into();
        }
        let statement = self.account_service.get_latest_statement(account_id).unwrap();
        to_response(Ok(statement.transactions))
    }

    fn get_transaction(&mut self, request: &Request) -> iron::Response {
        unimplemented!()
        //let transactions = try!(self.get_transactions(account_id, session_token));
        //let mut txs: Vec<Transaction> = transactions.into_iter().filter(|a| a.id == transaction_id).collect();
        //match txs.pop() {
        //    Some(tx) => Ok(tx),
        //    None => Err(ApiError::not_found("Transaction"))
        //}
    }
}

pub fn get_account_id(request: &Request) -> Option<Id> {
    let mut url = request.url.path();//.clone();
    let id = match url[1] {//.pop().unwrap() {
        id_string => {
            println!("poop {:?}", id_string);
            Id(i32::from_str_radix(id_string, 10).unwrap())
        }
    };
    Some(id)
}

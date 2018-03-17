use api::{AccountApiTrait, ApiResult, ApiError, ErrorType};
use db::{ConnectionSource, PostgresHelper, CambioError};
use domain::{Account, Id, Transaction, Session};
use hyper::mime::{Mime};
use iron::headers::{Cookie, Authorization, Bearer};
use iron::request::Request;
use iron;
use repositories::{AccountRepository, SessionRepository, UserRepository};
use repository::RepoRead;
use repository;
use serde_json;
use serde::Serialize;
use services::AccountService;

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
        if session.is_valid() {
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

    fn get_account(&mut self, account_id: Id, session_token: &str) 
        -> ApiResult<Account> {
        let clause = repository::UserClause::Id(account_id);
        let mut accounts = try!(self.account_repo.read(&clause));
        let session = try!(self.session_repo.read(&clause)).pop().unwrap();
        match accounts.pop() {
            Some(account) => {
                let owner_user_id = account.owner_user_id.unwrap();
                let user_clause = repository::UserClause::Id(owner_user_id);
                let account_user = try!(self.user_repo.read(&user_clause)).pop().unwrap();
                if session.email_address != Some(account_user.email_address) {
                    unimplemented!()
                }
                Ok(account)
            }, 
            None => {
                Err(ApiError::not_found("Account"))
            }
        }
    }

    fn get_transactions(&mut self, account_id: Id, session_token: &str) 
        -> ApiResult<Vec<Transaction>> {
        let account = try!(self.get_account(account_id, session_token));
        let statement = try!(self.account_service.get_latest_statement(account_id));
        Ok(statement.transactions)
    }

    fn get_transaction(&mut self, account_id: Id, transaction_id: Id, session_token: &str) 
        -> ApiResult<Transaction> {
        let transactions = try!(self.get_transactions(account_id, session_token));
        let mut txs: Vec<Transaction> = transactions.into_iter().filter(|a| a.id == transaction_id).collect();
        match txs.pop() {
            Some(tx) => Ok(tx),
            None => Err(ApiError::not_found("Transaction"))
        }
    }
}

pub fn get_session_token(r: &Request) -> Option<String> {
    let authorization:Option<&Authorization<Bearer>> = r.headers.get();
    match authorization {
        Some(ref bearer) => return Some(bearer.token.to_owned()),
        None => {}
    }
    let cookies_match: Option<&Cookie> = r.headers.get();
    if cookies_match.is_none() {
        return None;
    }
    let cookie_header = cookies_match.unwrap();
    for cookie in cookie_header.0.iter() {
        let cookie_bits: Vec<String> = cookie.clone().split("=").map(|s| s.to_owned()).collect();
        if cookie_bits[0] == "session_token" {
            let token = cookie_bits[1].clone();
            return Some(token);
        }
    }
    None
}

fn to_response<E: Serialize>(result: Result<E, CambioError>) -> iron::Response {
    let content_type = "application/json".parse::<Mime>().unwrap();
    match result {
        Ok(response_obj) => {
            let response_json = serde_json::to_string(&response_obj).unwrap();
            iron::Response::with((iron::status::Ok, response_json, content_type))
        },
        Err(err) => {
            let api_error: ApiError = err.into();
            api_error.into()
        }
    }
}


use api::api_init::ApiInit;
use api::{AccountApiTrait, AccountApiImpl, ApiError};
use db::{PostgresHelper};
use iron::headers::Cookie;
use iron::request::Request;
use iron;
use router::Router;
use std::borrow::Borrow;
use std::sync::Arc;
use hyper::mime::{Mime};
use serde_json;

#[derive(Clone)]
pub struct AccountApiInit<T: PostgresHelper> {
    helper: T
}

impl<T: PostgresHelper> AccountApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self {
            helper: helper
        }
    }
}

impl<T: PostgresHelper> ApiInit for AccountApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        let accounts_helper: Arc<T> = Arc::new(self.helper.clone());
        let account_helper: Arc<T> = Arc::new(self.helper.clone());
        let transaction_helper: Arc<T> = Arc::new(self.helper.clone());
        let transactions_helper: Arc<T> = Arc::new(self.helper.clone());

        router.get(
            "/accounts/",
            move |r: &mut Request| {
                let session_token_match = get_session_token(r);
                let session_token = match session_token_match {
                    Some(t) => t,
                    None => return Ok(ApiError::unauthorised().into())
                };
                let this_helper_ref: &T = transaction_helper.borrow();
                let mut api = AccountApiImpl::new(this_helper_ref.clone());
                let content_type = "application/json".parse::<Mime>().unwrap();
                Ok(match api.get_accounts(&session_token) {
                    Ok(accounts) => {
                        let response_json = serde_json::to_string(&accounts).unwrap();
                        iron::Response::with((iron::status::Ok, response_json, content_type))
                    },
                    Err(err) => {
                        err.into()
                    }
                })
            },
            "get_account",
        );

        router.get(
            "/accounts/:account_id",
            move |r: &mut Request| {
                unimplemented!()
            },
            "get_accounts",
        );

        router.get(
            "/accounts/:account_id/transactions/",
            move |r: &mut Request| {
                unimplemented!()
            },
            "get_transactions",
        );

        router.get(
            "/accounts/:account_id/transactions/:transaction_id",
            move |r: &mut Request| {
                unimplemented!()
            },
            "get_transaction",
        );
    }
}

pub fn get_session_token(r: &Request) -> Option<String> {
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




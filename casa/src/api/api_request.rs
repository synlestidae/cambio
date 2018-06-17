use api;
use db;
use domain;
use std::convert::TryFrom;
use iron::request::Request;
use bodyparser;
use hyper::mime::Mime;
use serde::Deserialize;
use serde_json;
use iron::prelude::*;
use std::error::Error;
use hyper::method::Method;
use api::{UserRequest, OrderApiRequest, AccountRequest, SettlementRequest, PaymentRequest};

#[derive(Debug)]
pub enum ApiRequest {
    User(UserRequest),
    Order(OrderApiRequest),
    Account(AccountRequest),
    Settlement(SettlementRequest),
    Payment(PaymentRequest),
}

impl ApiRequest {
    fn get_method(&self) -> Method {
        match self {
            ApiRequest::User(..) => Method::Post,
            ApiRequest::Account(..) => Method::Get,
            ApiRequest::Order(OrderApiRequest::GetActiveOrders) => Method::Get,
            ApiRequest::Order(OrderApiRequest::GetUserOrders) => Method::Get, 
            ApiRequest::Order(OrderApiRequest::PostNewOrder(..)) => Method::Post,
            ApiRequest::Order(OrderApiRequest::PostBuyOrder(..)) => Method::Post,
            ApiRequest::Settlement(..)=> Method::Post,
            ApiRequest::Payment(..)=> Method::Post,
        }
    }

    pub fn requires_auth(&self) -> bool {
        match self {
            ApiRequest::User(_) => false,
            _ => true
        }
    }
}

impl<'a, 'b, 'c> TryFrom<&'c mut Request<'a, 'b>> for ApiRequest {
    type Error = api::ApiError;
    fn try_from(request: &mut Request<'a, 'b>) -> Result<Self, Self::Error> {
        let url = request.url.clone();
        let mut path = url.path();
        if path.len() > 0 && path[path.len() - 1] == "" {
            drop(path.pop());
        }
        let request_obj = match path.as_slice() {
            &["users", "register"] => ApiRequest::User(UserRequest::Register(try!(get_api_obj(request)))),
            &["users", "log_in"] => ApiRequest::User(UserRequest::LogIn(try!(get_api_obj(request)))),
            &["orders", "active"] => ApiRequest::Order(OrderApiRequest::GetActiveOrders),
            &["orders", "me"] => ApiRequest::Order(OrderApiRequest::GetUserOrders),
            &["orders", "new"] => ApiRequest::Order(OrderApiRequest::PostNewOrder(try!(get_api_obj(request)))),
            &["orders", "buy"] => ApiRequest::Order(OrderApiRequest::PostBuyOrder(try!(get_api_obj(request)))),
            &["accounts"] => ApiRequest::Account(AccountRequest::GetAccounts),
            &["account", id] => ApiRequest::Account(AccountRequest::GetAccount(try!(serde_json::from_str(id)))),
            &["accounts", id, "transactions"] => {
                let tx_req = AccountRequest::GetAccountTransactions(try!(serde_json::from_str(id)));
                ApiRequest::Account(tx_req)
            },
            &["order", id, "settlement", "auth"] => {
                let order_id = try!(serde_json::from_str(id));
                let cred = try!(get_api_obj(request));
                let s_req = SettlementRequest::PostSettlementEthAuth(order_id, cred);
                ApiRequest::Settlement(s_req)
            },
            &["payment"] => ApiRequest::Payment(PaymentRequest::CreditCardPayment(try!(get_api_obj(request)))),
            _ => return Err(api::ApiError::not_found_path(&path.into_iter().collect::<Vec<_>>().join("/")))
        };
        let expected_method = request_obj.get_method();
        if  expected_method == request.method {
            Ok(request_obj)
        } else {
            Err(api::ApiError::bad_method(expected_method))
        }
    }
}

fn get_api_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, api::ApiError>
where
    for<'a> T: Deserialize<'a>,
{
    let content_type = "application/json".parse::<Mime>().unwrap();
    match request.get_ref::<bodyparser::Struct<T>>() {
        Ok(&Some(ref body_obj)) => Ok(body_obj.clone()),
        Ok(&None) => {
            Err(api::ApiError::bad_format("Body of HTTP request cannot be empty"))
        }
        Err(error) => {
            Err(api::ApiError::bad_format(error.description()))
        }
    }
}

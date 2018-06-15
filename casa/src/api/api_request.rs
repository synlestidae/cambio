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

pub enum ApiRequest {
    // Users
    Register(api::Registration), 
    LogIn(api::LogIn), 
    // Accounts
    GetAccounts,
    GetAccount(domain::AccountId),
    GetAccountTransactions(domain::AccountId),
    GetAccountTransaction(domain::AccountId, domain::TransactionId),
    // Orders
    GetActiveOrders,
    GetUserOrders, 
    PostNewOrder(api::OrderRequest),
    PostBuyOrder(api::OrderBuy)
}

impl ApiRequest {
    fn get_method(&self) -> Method {
        match self {
            // Users
            ApiRequest::Register(_) => Method::Post, 
            ApiRequest::LogIn(_) => Method::Post, 
            // Accounts
            ApiRequest::GetAccounts => Method::Get,
            ApiRequest::GetAccount(_) => Method::Get,
            ApiRequest::GetAccountTransactions(_) => Method::Get,
            ApiRequest::GetAccountTransaction(..) => Method::Get,
            // Orders
            ApiRequest::GetActiveOrders => Method::Get,
            ApiRequest::GetUserOrders => Method::Get, 
            ApiRequest::PostNewOrder(_) => Method::Post,
            ApiRequest::PostBuyOrder(_) => Method::Post
        }
    }
}

impl<'a, 'b> TryFrom<Request<'a, 'b>> for ApiRequest {
    type Error = api::ApiError;
    fn try_from(mut request: Request<'a, 'b>) -> Result<Self, Self::Error> {
        let url = request.url.clone();
        let path = url.path();
        let request_obj = match path.as_slice() {
            &["users", "register"] => ApiRequest::Register(try!(get_api_obj(&mut request))),
            &["users", "log_in"] => ApiRequest::LogIn(try!(get_api_obj(&mut request))),
            &["orders", "active"] => ApiRequest::GetActiveOrders,
            &["orders", "me"] => ApiRequest::GetUserOrders,
            &["orders", "new"] => ApiRequest::PostNewOrder(try!(get_api_obj(&mut request))),
            &["orders", "buy"] => ApiRequest::PostBuyOrder(try!(get_api_obj(&mut request))),
            _ => unimplemented!()
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

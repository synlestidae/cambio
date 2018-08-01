use api;
use api::{
    AccountRequest, ApiError, OrderApiRequest, PaymentRequest, SettlementRequest, UserRequest,
};
use db;
use domain;
use hyper::header::ContentType;
use hyper::method::Method;
use hyper::mime::Mime;
use iron::prelude::*;
use iron::request::Request;
use serde::Deserialize;
use serde_json;
use serde_urlencoded;
use std::convert::TryFrom;
use std::error::Error;
use std::io::Read;
use std::str::FromStr;

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
            ApiRequest::User(UserRequest::GetPersonalDetails) => Method::Get,
            ApiRequest::User(UserRequest::SetPersonalDetails(..)) => Method::Post,
            ApiRequest::User(..) => Method::Post,
            ApiRequest::Account(..) => Method::Get,
            ApiRequest::Order(OrderApiRequest::GetActiveOrders) => Method::Get,
            ApiRequest::Order(OrderApiRequest::GetChangedOrders(..)) => Method::Get,
            ApiRequest::Order(OrderApiRequest::GetUserOrders) => Method::Get,
            ApiRequest::Order(OrderApiRequest::PostNewOrder(..)) => Method::Post,
            ApiRequest::Order(OrderApiRequest::PostBuyOrder(..)) => Method::Post,
            ApiRequest::Settlement(..) => Method::Post,
            ApiRequest::Payment(..) => Method::Post,
        }
    }

    pub fn requires_auth(&self) -> bool {
        match self {
            ApiRequest::User(_) => false,
            _ => true,
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
            &["users", "register"] => {
                ApiRequest::User(UserRequest::Register(try!(get_api_obj(request))))
            }
            &["users", "register", "new_confirmation_email"] => {
                ApiRequest::User(UserRequest::ResendEmail(try!(get_api_obj(request))))
            }
            &["users", "log_in"] => {
                ApiRequest::User(UserRequest::LogIn(try!(get_api_obj(request))))
            }
            &["users", "confirm"] => {
                ApiRequest::User(UserRequest::Confirm(try!(get_api_obj(request))))
            }
            &["users", "personal", "details"] => {
                if (request.method == Method::Get) {
                    ApiRequest::User(UserRequest::GetPersonalDetails)
                } else if (request.method == Method::Post) {
                    ApiRequest::User(UserRequest::SetPersonalDetails(get_api_obj(request)?))
                } else {
                    return Err(api::ApiError::bad_method(Method::Get));
                }
            },
            &["orders", "active"] => ApiRequest::Order(OrderApiRequest::GetActiveOrders),
            &["orders", "changed"] => ApiRequest::Order(OrderApiRequest::GetChangedOrders(get_url_obj(request)?)),
            &["orders", "me"] => ApiRequest::Order(OrderApiRequest::GetUserOrders),
            &["orders", "new"] => {
                ApiRequest::Order(OrderApiRequest::PostNewOrder(try!(get_api_obj(request))))
            }
            &["orders", "buy"] => {
                ApiRequest::Order(OrderApiRequest::PostBuyOrder(try!(get_api_obj(request))))
            }
            &["accounts"] => ApiRequest::Account(AccountRequest::GetAccounts),
            &["account", id] => {
                let account_id =
                    domain::AccountId::from_str(id).map_err(|_| ApiError::not_found("Account ID"))?;
                ApiRequest::Account(AccountRequest::GetAccount(account_id))
            }
            &["accounts", id, "transactions"] => {
                let account_id =
                    domain::AccountId::from_str(id).map_err(|_| ApiError::not_found("Account ID"))?;
                let tx_req = AccountRequest::GetAccountTransactions(account_id);
                ApiRequest::Account(tx_req)
            }
            &["order", id, "settlement", "auth"] => {
                let order_id =
                    domain::OrderId::from_str(id).map_err(|_| ApiError::not_found("Account ID"))?;
                let cred = try!(get_api_obj(request));
                let s_req = SettlementRequest::PostSettlementEthAuth(order_id, cred);
                ApiRequest::Settlement(s_req)
            }
            &["payment"] => {
                let payment_request: PaymentRequest = try!(get_api_obj(request));
                ApiRequest::Payment(payment_request)
            },
            _ => {
                return Err(api::ApiError::not_found_path(&path
                    .into_iter()
                    .collect::<Vec<_>>()
                    .join("/")))
            }
        };
        let expected_method = request_obj.get_method();
        if expected_method == request.method {
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
    let json_mime_type = "application/json".parse::<Mime>().unwrap();
    let form_mime_type = "application/x-www-form-urlencoded".parse::<Mime>().unwrap();
    let headers_copy = request.headers.clone();
    let header = headers_copy.get::<ContentType>();

    if let Some(m) = header {
        if m.0 == json_mime_type {
            get_json_obj(request)
        } else if m.0 == form_mime_type {
            get_form_obj(request)
        } else {
            Err(api::ApiError::bad_format("Unsupported Content-Type"))
        }
    } else {
        Err(api::ApiError::bad_format("Missing `Content-Type` header"))
    }
}

fn get_json_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, api::ApiError>
where
    for<'a> T: Deserialize<'a>,
{
    let mut bytes = Vec::new();
    try!(request.body.read_to_end(&mut bytes));
    if bytes.len() == 0 {
        Err(api::ApiError::bad_format(
            "Body of HTTP request cannot be empty",
        ))
    } else {
        let val: T = serde_json::from_slice(&bytes)?;
        Ok(val)
    }
    /*match request.get_ref::<bodyparser::Struct<T>>() {
        Ok(&Some(ref body_obj)) => Ok(body_obj.clone()),
        Ok(&None) => ,
        Err(error) => Err(api::ApiError::bad_format(error.description())),
    }*/
}

fn get_form_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, api::ApiError>
where
    for<'a> T: Deserialize<'a>,
{
    let mut bytes = Vec::new();
    try!(request.body.read_to_end(&mut bytes));
    let obj: T = try!(serde_urlencoded::from_bytes(&bytes));
    Ok(obj)
}

fn get_url_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, api::ApiError>
where
    for<'a> T: Deserialize<'a>,
{
    let bytes = if let Some(query_str) = request.url.query() {
        query_str.to_string().into_bytes()
    } else {
        vec![]
    };
    let obj: T = serde_urlencoded::from_bytes(&bytes)?;
    Ok(obj)
}

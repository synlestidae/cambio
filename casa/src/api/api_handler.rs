use db;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::IronResult;
use api::ApiRequest;
use std::convert::TryFrom;

pub struct ApiHandler<T: db::PostgresHelper> {
    db: T
}

impl<T: db::PostgresHelper + 'static> ApiHandler<T> {
    pub fn new(db: T) -> Self {
        Self {
            db: db
        }
    }
}

impl<T: db::PostgresHelper + 'static> Handler for ApiHandler<T> {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        let api_request_result: Result<ApiRequest, _> = TryFrom::try_from(request);
        let api_request = match api_request_result {
            Ok(r) => r,
            Err(err) => return Ok(err.into())
        };

        /*match api_request {
            // Users
            ApiRequest::Register(..) => unimplemented!(),
            ApiRequest::LogIn(..) => unimplemented!(),
            // Accounts
            ApiRequest::GetAccounts => unimplemented!(),
            ApiRequest::GetAccount(..) => unimplemented!(),
            ApiRequest::GetAccountTransactions(..) => unimplemented!(),
            ApiRequest::GetAccountTransaction(..) => unimplemented!(),
            // Orders
            ApiRequest::GetActiveOrders => unimplemented!(),
            ApiRequest::GetUserOrders => unimplemented!(),
            ApiRequest::PostNewOrder(..) => unimplemented!(),
            ApiRequest::PostBuyOrder(..) => unimplemented!(),
            ApiRequest::PostSettlementEthAuth(..) => unimplemented!(),
            ApiRequest::GetSettlementStatus => unimplemented!(),
        }*/
        unimplemented!()
    }
}

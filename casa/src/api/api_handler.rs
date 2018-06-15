use db;
use api;
use domain;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::IronResult;
use api::ApiRequest;
use std::convert::TryFrom;
use api::{ UserApiTrait, UserApi, SessionTokenSource, UserRequest, OrderApiRequest, OrderApiImpl, OrderApiTrait};
use repository::Readable;

pub struct ApiHandler<T: db::PostgresHelper> {
    db: T,
    web3_address: String
}

impl<T: db::PostgresHelper + 'static> ApiHandler<T> {
    pub fn new(db: T, web3_address: &str) -> Self {
        Self {
            db: db,
            web3_address: web3_address.to_owned()
        }
    }
}

impl<T: db::PostgresHelper + 'static> Handler for ApiHandler<T> {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        let mut db = self.db.clone();
        let user: domain::User = match request.get_session_token() {
            Some(token) => {
                match token.get(&mut db) {
                    Ok(user) => {
                        match user.user_id.get(&mut db) {
                            Ok(user) => user,
                            Err(err) => return Ok(err.into())
                        }
                    },
                    Err(err) => return Ok(err.into())
                }
            },
            None => {
                // just make a fake user that will never get used
                domain::User { 
                    id: None, 
                    email_address: "".to_owned(),
                    password: None,
                    password_hash: None,
                    owner_id: None
                }
            }
        };
        let api_request_result: Result<ApiRequest, _> = TryFrom::try_from(request);
        let api_request = match api_request_result {
            Ok(r) => r,
            Err(err) => return Ok(err.into())
        };

        if api_request.requires_auth() && user.id.is_none() {
            return Ok(api::ApiError::unauthorised().into())
        }
        let response = match api_request {
            ApiRequest::User(user_request) => {
                let mut user_api = UserApi::new(db.clone(), &self.web3_address);
                match user_request {
                    UserRequest::Register(reg) => user_api.put_register(&reg),
                    UserRequest::LogIn(login) => user_api.post_log_in(&login)
                }
            },
            ApiRequest::Order(order_request) => {
                let mut order_api = OrderApiImpl::new(db.clone()); 
                match order_request {
                    OrderApiRequest::GetActiveOrders => order_api.get_active_orders(),
                    OrderApiRequest::GetUserOrders => order_api.get_user_orders(&user), 
                    OrderApiRequest::PostNewOrder(new_order) => order_api.post_new_order(&user, &new_order),
                    OrderApiRequest::PostBuyOrder(order_buy) => order_api.post_buy_order(&user, &order_buy),
                }
            },
            ApiRequest::Account(..) => unimplemented!(),
            ApiRequest::Settlement(..) => unimplemented!(),
        };

        Ok(response)
    }
}

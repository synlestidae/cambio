use db;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::IronResult;
use api::ApiRequest;
use std::convert::TryFrom;
use api::{UserRequest, UserApiTrait, UserApi};

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
        let api_request_result: Result<ApiRequest, _> = TryFrom::try_from(request);
        let api_request = match api_request_result {
            Ok(r) => r,
            Err(err) => return Ok(err.into())
        };

        let response = match api_request {
            ApiRequest::User(user_request) => {
                let mut user_api = UserApi::new(self.db.clone(), &self.web3_address);
                match user_request {
                    UserRequest::Register(reg) => user_api.put_register(&reg),
                    UserRequest::LogIn(login) => user_api.post_log_in(&login)
                }
            },
            ApiRequest::Order(..) => unimplemented!(),
            ApiRequest::Account(..) => unimplemented!(),
            ApiRequest::Settlement(..) => unimplemented!(),
        };

        Ok(response)
    }
}

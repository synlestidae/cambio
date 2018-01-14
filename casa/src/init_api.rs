use api::{UserApi, UserApiTrait, ApiError};
use db::{PostgresSource, PostgresHelperImpl, ConnectionSource};
use iron;
use api::Registration;
use hyper;
use serde::Serialize;
use iron::request::Request;
use std::clone::Clone;
use serde::Deserialize;
use router::Router;
use bodyparser;
use serde_json;
use iron::prelude::*;

use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

struct IronApi {
    user_api: UserApi<PostgresHelperImpl>
}

impl IronApi {
    pub fn new(conn_source: PostgresSource) -> Self {
        let helper = PostgresHelperImpl::new(conn_source); 
        IronApi {
            user_api: UserApi::new(helper)
        }
    }

    fn init_user_api(&mut self, router: &mut Router) {
        // self.handle_put_register(request)
        router.put("/users/register", |request: &mut Request| unimplemented!(), "put_register");
    }

    fn handle_put_register(&mut self, request: &mut Request) -> iron::IronResult<iron::Response> {
        let content_type = "application/json".parse::<Mime>().unwrap();
        let registration: Result<Registration, ()> = get_api_obj(request);
        let response: iron::Response;

        if let Ok(registration) = get_api_obj(request) {
            response = match self.user_api.put_register(registration) {
                Ok(response_obj) => {
                    let response_json = serde_json::to_string(&response_obj).unwrap();
                    iron::Response::with((iron::status::Ok, response_json, content_type))
                },
                Err(err_obj) => {
                    let err_json = serde_json::to_string(&err_obj).unwrap();
                    iron::Response::with((iron::status::InternalServerError, err_json, content_type))
                }
            }
        } else {
            let err_obj = ApiError::internal("Failed to parse the JSON supplied");
            let err_json = serde_json::to_string(&err_obj).unwrap();
            return Ok(iron::Response::with((iron::status::BadRequest, 
                 err_json, 
                 content_type)));
        }

        Ok(response)
    }

    fn post_log_in() {
    }
}


fn get_api_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, ()> 
where for<'a> T: Deserialize<'a> {
    let result: Result<T, ()> = match request.get_ref::<bodyparser::Struct<T>>() {
        Ok(&Some(ref body_obj)) => Ok(body_obj.clone()),
        _ => Err(())
    };

    result
}

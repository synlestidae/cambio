use api::{Registration, Profile, ApiResult, ApiError, LogIn, UserApiTrait, get_api_obj};
use iron::{Request, Response};
use db::{ConnectionSource, PostgresHelper};
use services::UserService;
use domain::{User, Session};
use hyper::mime::Mime;
use serde_json;
use iron;

#[derive(Clone)]
pub struct UserApi<C: PostgresHelper> {
    user_service: UserService<C>,
}

impl<C: PostgresHelper> UserApi<C> {
    pub fn new(helper: C) -> Self {
        Self { user_service: UserService::new(helper) }
    }
}

impl<C: PostgresHelper> UserApiTrait for UserApi<C> {
    fn put_register(&mut self, request: &mut Request) -> Response {
        debug!("Parsing request body");
        let registration: Registration = match get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };

        // test password requirements
        if registration.password.len() < 8 {
            return ApiError::bad_format("Password needs to be at least 8 characters").into();
        }

        debug!("Calling register_user function");

        let register_result = self.user_service.register_user(
            &registration.email_address,
            registration.password,
        );

        const GENERIC_FAIL_MSG: &str = "Failed to register user";

        match register_result {
            Err(error) => ApiError::unknown(GENERIC_FAIL_MSG).into(),
            Ok(user) => {
                let response_json = serde_json::to_string(&user).unwrap();
                let content_type = "application/json".parse::<Mime>().unwrap();
                iron::Response::with((iron::status::Ok, response_json, content_type))
            }
        }
    }

    fn post_log_in(&mut self, request: &mut Request) -> Response {
        let log_in: LogIn = match get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };
        let log_in_result = self.user_service.log_user_in(
            &log_in.email_address,
            log_in.password,
        );
        match log_in_result {
            Ok(session) => {
                let response_json = serde_json::to_string(&session).unwrap();
                let content_type = "application/json".parse::<Mime>().unwrap();
                iron::Response::with((iron::status::Ok, response_json, content_type))
            },
            Err(error) => ApiError::unknown("Could not log you in").into(),
        }
    }

    fn get_profile(&mut self, request: &mut Request) -> Response {
        let user: User = match get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };

        unimplemented!()
    }
}

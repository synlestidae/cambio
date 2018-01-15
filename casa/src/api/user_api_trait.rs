use iron::{Request, Response};
use api::{Registration, Profile, ApiResult, ApiError, LogIn};
use domain::{User, Session};

pub trait UserApiTrait: Clone {
    fn put_register(&mut self, request: &mut Request) -> Response;
    fn post_log_in(&mut self, request: &mut Request) -> Response;
    fn get_profile(&mut self, request: &mut Request) -> Response;
}

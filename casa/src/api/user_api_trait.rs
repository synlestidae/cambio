use iron::{Request, Response};
use api::{ApiError, ApiResult, LogIn, Profile, Registration};
use domain::{Session, User};

pub trait UserApiTrait: Clone {
    fn put_register(&mut self, request: &mut Request) -> Response;
    fn post_log_in(&mut self, request: &mut Request) -> Response;
    fn get_profile(&mut self, request: &mut Request) -> Response;
}

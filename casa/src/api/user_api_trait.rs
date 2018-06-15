use api::{ApiError, ApiResult, LogIn, Profile, Registration};
use domain::{Session, User};
use iron::{Request, Response};
use api;

pub trait UserApiTrait: Clone {
    fn put_register(&mut self, reg: &api::Registration) -> Response;
    fn post_log_in(&mut self, login: &api::LogIn) -> Response;
    fn get_profile(&mut self, user: &User) -> Response;
}

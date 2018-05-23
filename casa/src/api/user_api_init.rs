use api::api_init::ApiInit;
use api::Registration;
use api::{ApiError, UserApi, UserApiTrait};
use bodyparser;
use db::{ConnectionSource, PostgresHelper, PostgresHelperImpl, PostgresSource};
use hyper;
use hyper::header::{ContentType, Headers};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use iron;
use iron::request::Request;
use iron::Handler;
use router::Router;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std;
use std::borrow::Borrow;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserApiInit<T: PostgresHelper> {
    helper: T,
    web3_address: String,
}

impl<T: PostgresHelper> UserApiInit<T> {
    pub fn new(helper: T, web3_address: &str) -> Self {
        Self {
            helper: helper,
            web3_address: web3_address.to_owned(),
        }
    }
}

impl<T: PostgresHelper> ApiInit for UserApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        let register_helper: Arc<T> = Arc::new(self.helper.clone());
        let log_in_helper: Arc<T> = Arc::new(self.helper.clone());
        let profile_helper: Arc<T> = Arc::new(self.helper.clone());

        let a1 = self.web3_address.to_owned();
        let a2 = self.web3_address.to_owned();
        let a3 = self.web3_address.to_owned();

        router.post(
            "/users/register/",
            move |r: &mut Request| {
                let this_helper_ref: &T = register_helper.borrow();
                let mut api = UserApi::new(this_helper_ref.clone(), &a1);
                Ok(api.put_register(r))
            },
            "put_register",
        );

        router.post(
            "/users/log_in/",
            move |r: &mut Request| {
                let this_helper_ref: &T = log_in_helper.borrow();
                let mut api = UserApi::new(this_helper_ref.clone(), &a2);
                Ok(api.post_log_in(r))
            },
            "post_log_in",
        );

        router.get(
            "/users/profile/",
            move |r: &mut Request| {
                let profile_helper_ref: &T = profile_helper.borrow();
                let mut api = UserApi::new(profile_helper_ref.clone(), &a3);
                Ok(api.get_profile(r))
            },
            "get_profile",
        );
    }
}

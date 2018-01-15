use api::Registration;
use api::api_init::ApiInit;
use std::sync::Arc;
use api::{UserApi, UserApiTrait, ApiError};
use bodyparser;
use db::{PostgresSource, PostgresHelper, PostgresHelperImpl, ConnectionSource};
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use hyper;
use iron::prelude::*;
use iron::request::Request;
use iron::Handler;
use iron;
use router::Router;
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::clone::Clone;
use std;

#[derive(Clone)]
pub struct UserApiInit<T: PostgresHelper> {
    helper: T,
}

impl<T: PostgresHelper> UserApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self { helper: helper }
    }
}

impl<T: PostgresHelper> ApiInit for UserApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        let helper = Box::new(Arc::new(self.helper.clone()));
        let helper2 = Box::new(Arc::new(self.helper.clone()));

        router.put(
            "/users/register/",
            move |r: &mut Request| {
                let mut api = UserApi::new((*(*helper)).clone());
                Ok(api.put_register(r))
            },
            "put_register",
        );

        router.post(
            "/users/log_in/",
            move |r: &mut Request| {
                let mut api = UserApi::new((*(*helper2)).clone());
                Ok(api.post_log_in(r))
            },
            "post_log_in",
        );
    }
}

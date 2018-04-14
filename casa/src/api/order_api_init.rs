use api::api_init::ApiInit;
//use api::{AccountApiTrait, AccountApiImpl, ApiError};
use db::{PostgresHelper};
use iron::headers::{Cookie, Authorization, Bearer};
use iron::request::Request;
use iron;
use router::Router;
use std::borrow::Borrow;
use std::sync::Arc;
use serde_json;

#[derive(Clone)]
pub struct OrderApiInit<T: PostgresHelper> {
    helper: T
}

impl<T: PostgresHelper> OrderApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self {
            helper: helper
        }
    }
}

impl<T: PostgresHelper> ApiInit for OrderApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        router.get(
            "/orders/active/",
            move |r: &mut Request| {
                unimplemented!()
            },
            "get_active_orders",
        );

        router.get(
            "/orders/me/",
            move |r: &mut Request| {
                unimplemented!()
            },
            "get_user_orders",
        );

        router.post(
            "/orders/new_order",
            move |r: &mut Request| {
                unimplemented!()
            },
            "post_new_order",
        );

        router.post(
            "/orders/buy/:id",
            move |r: &mut Request| {
                unimplemented!()
            },
            "post_buy_order",
        );
    }
}

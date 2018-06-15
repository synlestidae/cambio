use api::{ApiInit, OrderApiImpl, OrderApiTrait};
use db;
use iron;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;
use router::Router;
use serde_json;
use std::borrow::Borrow;
use std::sync::Arc;

#[derive(Clone)]
pub struct OrderApiInit<T: db::PostgresHelper> {
    helper: T,
}

impl<T: db::PostgresHelper> OrderApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self { helper: helper }
    }
}

impl<T: db::PostgresHelper> ApiInit for OrderApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        let active_orders_helper: Arc<T> = Arc::new(self.helper.clone());
        let my_orders_helper: Arc<T> = Arc::new(self.helper.clone());
        let new_order_helper: Arc<T> = Arc::new(self.helper.clone());
        let buy_order_helper: Arc<T> = Arc::new(self.helper.clone());

        router.get(
            "/orders/active/",
            move |r: &mut Request| {
                let this_helper_ref: &T = active_orders_helper.borrow();
                let mut api: OrderApiImpl<T> = OrderApiImpl::new(this_helper_ref.clone());
                Ok(api.get_active_orders(r))
            },
            "get_active_orders",
        );

        router.get(
            "/orders/me/",
            move |r: &mut Request| {
                let this_helper_ref: &T = my_orders_helper.borrow();
                let mut api: OrderApiImpl<T> = OrderApiImpl::new(this_helper_ref.clone());
                Ok(api.get_user_orders(r))
            },
            "get_user_orders",
        );

        router.post(
            "/orders/new",
            move |r: &mut Request| {
                let this_helper_ref: &T = new_order_helper.borrow();
                let mut api: OrderApiImpl<T> = OrderApiImpl::new(this_helper_ref.clone());
                Ok(api.post_new_order(r))
            },
            "post_new_order",
        );

        router.post(
            "/orders/buy/:id",
            move |r: &mut Request| {
                let this_helper_ref: &T = buy_order_helper.borrow();
                let mut api: OrderApiImpl<T> = OrderApiImpl::new(this_helper_ref.clone());
                Ok(api.post_buy_order(r))
            },
            "post_buy_order",
        );
    }
}

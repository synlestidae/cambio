use iron;
use api;
use domain;

pub trait OrderApiTrait {
    fn get_active_orders(&mut self) -> iron::Response;
    fn get_user_orders(&mut self, user: &domain::User) -> iron::Response;
    fn post_new_order(&mut self, user: &domain::User, order: &api::OrderRequest) -> iron::Response;
    fn post_buy_order(&mut self, user: &domain::User, request: &api::OrderBuy) -> iron::Response;
}

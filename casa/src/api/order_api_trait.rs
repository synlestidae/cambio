use iron;

pub trait OrderApiTrait {
    fn get_active_orders(&mut self, request: &iron::Request) -> iron::Response;
    fn get_user_orders(&mut self, request: &iron::Request) -> iron::Response;
    fn post_new_order(&mut self, request: &iron::Request) -> iron::Response;
    fn post_buy_order(&mut self, request: &iron::Request) -> iron::Response;
}

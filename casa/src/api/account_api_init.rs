use db::{PostgresHelper};
use iron::request::Request;
use router::Router;
use api::api_init::ApiInit;

#[derive(Clone)]
pub struct AccountApiInit<T: PostgresHelper> {
    helper: T
}


impl<T: PostgresHelper> ApiInit for AccountApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
    }
}

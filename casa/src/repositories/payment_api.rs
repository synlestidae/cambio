use db::PostgresHelper;
use domain::{Payment, User};
use iron::response::Response;

pub struct PaymentApi<H: PostgresHelper> {
    db: H
}

impl<H: PostgresHelper> PaymentApi<H> {
    pub fn post_payment(&mut self, 
        user: &User,
        payment: &Payment) -> Response {
        unimplemented!()
    }
}

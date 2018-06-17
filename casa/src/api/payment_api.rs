use db::PostgresHelper;
use domain::{Payment, User};
use iron::response::Response;

pub struct PaymentApi<H: PostgresHelper + 'static> {
    db: H
}

impl<H: PostgresHelper + 'static> PaymentApi<H> {
    pub fn new(db: H) -> Self {
        Self {
            db: db
        }
    }

    pub fn post_payment(&mut self, 
        user: &User,
        payment: &Payment) -> Response {
        unimplemented!()
    }
}

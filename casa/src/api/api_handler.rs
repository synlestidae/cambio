use db;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::IronResult;

pub struct ApiHandler<T: db::PostgresHelper> {
    db: T
}

impl<T: db::PostgresHelper + 'static> ApiHandler<T> {
    pub fn new(db: T) -> Self {
        Self {
            db: db
        }
    }
}

impl<T: db::PostgresHelper + 'static> Handler for ApiHandler<T> {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        unimplemented!()
    }
}

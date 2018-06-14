use api::ApiInit;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;
use router::Router;

pub struct SettlementApiInit {
}

impl ApiInit for SettlementApiInit {
    fn init_api(&mut self, router: &mut Router) {
        unimplemented!()
    }
}

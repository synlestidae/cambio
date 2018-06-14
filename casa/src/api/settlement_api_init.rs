use api::ApiInit;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;
use router::Router;
use db;

pub struct SettlementApiInit<H: db::PostgresHelper> {
    db: H
}

impl<H: db::PostgresHelper> ApiInit for SettlementApiInit<H> {
    fn init_api(&mut self, router: &mut Router) {
        router.post(
            "/orders/:id/eth_credentials",
            move |r: &mut Request| {
                unimplemented!()
            },
            "post_settlement_eth_auth",
        );

        unimplemented!()
    }
}

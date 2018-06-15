use api::ApiInit;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;
use router::Router;
use api::SettlementApiImpl;
use db;
use api::settlement_api_trait::SettlementApiTrait;
use std::borrow::Borrow;
use std::sync::Arc;

pub struct SettlementApiInit<H: db::PostgresHelper + 'static> {
    db: H
}

impl<H: db::PostgresHelper> ApiInit for SettlementApiInit<H> {
    fn init_api(&mut self, router: &mut Router) {
        let auth_helper = Arc::new(self.db.clone());
        router.post(
            "/settlement/auth",
            move |r: &mut Request| {
                let helper: &H = auth_helper.borrow();
                let mut api: SettlementApiImpl<H> = 
                    SettlementApiImpl::new(helper.clone(), unimplemented!());
                Ok(api.post_settlement_eth_auth(r))
            },
            "post_settlement_eth_auth",
        );

        unimplemented!()
    }
}

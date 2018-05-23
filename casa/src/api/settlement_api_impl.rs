use api::SettlementApiTrait;
use api;
use db::PostgresHelper;
use db;
use domain;
use iron;
use repository::Retrievable;
use services;

pub struct SettlementApiImpl<C: PostgresHelper> {
    db: C
}

impl<C: PostgresHelper> SettlementApiTrait for SettlementApiImpl<C> {
    fn post_settlement_eth_auth(&mut self, request: &mut iron::Request) -> iron::Response {
        let credentials: api::SettlementEthCredentials = match api::get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };
        let settlement_id = credentials.settlement_id;

        // retrieve the settlement
        let settlement: domain::OrderSettlement = match settlement_id.get(&mut self.db) {
            Ok(s) => s,  
            Err(err) => return err.into()
        };

        // TODO extract the session id
        // TODO retrieve the user

        //let settlement = self.settlement_repo
        // check order owner is user
        unimplemented!()
    }

    fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

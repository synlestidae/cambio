use api;
use api::SessionTokenSource;
use api::SettlementApiTrait;
use db;
use db::PostgresHelper;
use domain;
use iron;
use repositories;
use db::TryFromRow;
use repository::RepoUpdate;
use query::Selectable;
use repository::Readable;
use services;
use web3::types::U256;
use std::sync::mpsc::Sender;
use jobs::JobRequest;

pub struct SettlementApiImpl<C: PostgresHelper> {
    db: C,
    job_tx: Sender<JobRequest>
}

impl<H: PostgresHelper> SettlementApiImpl<H> {
    pub fn new(db: H, job_tx: Sender<JobRequest>) -> Self {
        Self {
            db: db,
            job_tx: job_tx
        }
    }

}

impl<C: PostgresHelper> SettlementApiTrait for SettlementApiImpl<C> {
    fn post_settlement_eth_auth(&mut self, request: &mut iron::Request) -> iron::Response {
        let credentials: api::SettlementEthCredentials = match api::get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };
        let settlement_id = credentials.settlement_id;

        // retrieve the settlement
        let mut settlement: domain::OrderSettlement = match settlement_id.get(&mut self.db) {
            Ok(s) => s,
            Err(err) => return err.into(),
        };

        // TODO get session
        let session: domain::Session =
            match request.get_session_token().map(|s| s.get(&mut self.db)) {
                Some(Ok(token)) => token,
                _ => return db::CambioError::unauthorised().into(),
            };

        // TODO retrieve the user
        let user: domain::User = match unimplemented!() {
            _ => {
                return db::CambioError::shouldnt_happen(
                    "Unable to find your account.",
                    "Failed to find user for that session.",
                ).into()
            }
        };

        // retrieve the selling order's owner
        let owner: domain::User = match settlement.selling_order.owner_id.get(&mut self.db) {
            Ok(user) => user,
            Err(err) => return err.into(),
        };

        if owner.id != user.id {
            return api::ApiError::not_found("Settlement").into();
        }

        iron::response::Response::with((iron::status::Status::Ok, format!("")))
    }

    fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

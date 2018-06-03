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
use repository::Retrievable;
use services;

pub struct SettlementApiImpl<C: PostgresHelper> {
    db: C,
    eth_address: String,
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

        // now user is authorised to bring this settlement to the next stage
        let mut settlement_service =
            services::SettlementService::new(self.db.clone(), &self.eth_address);
        let mut settlement_repo = repositories::SettlementRepository::new(self.db.clone());
        settlement.settlement_status = domain::SettlementStatus::WaitingEth;
        settlement_repo.update(&settlement);

        // now settlement is marked as waiting on ethereum - we MUST do it now
        // TODO test that the ethereum connection is okay first
        let max_cost_wei = 854800000000000;
        let eth_tx = settlement_service
            .begin_eth_transfer(
                settlement.id.unwrap(),
                &credentials.unique_id,
                credentials.password,
                max_cost_wei,
            )
            .unwrap();

        // sweet! update that settlement now
        settlement.settlement_status = domain::SettlementStatus::Settled;
        settlement_repo.update(&settlement);

        //let settlement = self.settlement_repo
        // check order owner is user
        api::utils::to_response(Ok(settlement))
    }

    fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

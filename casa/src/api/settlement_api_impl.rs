use api::SettlementApiTrait;
use api;
use iron;
use repositories;
use repository;
use services;
use db::PostgresHelper;
use db;
use domain;
use repository::RepoRead;

pub struct SettlementApiImpl<C: PostgresHelper> {
    order_repo: repositories::OrderRepository<C>,
    settlement_repo: repositories::SettlementRepository<C>,
}

impl<C: PostgresHelper> SettlementApiTrait for SettlementApiImpl<C> {
    fn post_settlement_eth_auth(&mut self, request: &mut iron::Request) -> iron::Response {
        let credentials: api::SettlementEthCredentials = match api::get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };

        // TODO extract the session id
        // TODO retrieve the user

        // retrieve the settlement
        let settlement_clause = repository::UserClause::Id(credentials.settlement_id);
        let settlement_result = self.settlement_repo.read(&settlement_clause);
        let settlement = match settlement_result.map(|mut v| v.pop()) {
            Ok(Some(s)) => s,
            Ok(None) => return api::ApiError::not_found("Settlement").into(),
            Err(err) => return err.into(),
        };

        // check settlement status
        const EXPECTED_STATE: domain::SettlementStatus =
            domain::SettlementStatus::WaitingEthCredentials;
        if settlement.settlement_status != EXPECTED_STATE {
            let err = db::CambioError::not_permitted(
                "Settlement is not awaiting credentials.",
                &format!(
                    "Expected settlement status to be {:?}, got {:?}",
                    EXPECTED_STATE, settlement.settlement_status
                ),
            ).into();
            return err;
        }

        // retrieve the order for the eth transfer
        let eth_sell_order = if settlement.buying_order.sell_asset_type.is_crypto() {
            settlement.buying_order
        } else if settlement.buying_order.buy_asset_type.is_crypto() {
            settlement.selling_order
        } else {
            return db::CambioError::not_permitted(
                "Settlement doesn't have an Ethereum-buying order. An exchange cannot take place.",
                "Neither order in the settlement is selling Ethereum.",
            ).into();
        };

        if eth_sell_order.id != Some(credentials.order_id) {
            return db::CambioError::not_permitted(
                "Invalid combination of settlement or order ID.",
                "The order ID does not match any order for settlement.",
            ).into();
        }

        // now retrieve the user

        //let settlement = self.settlement_repo
        // check order owner is user
        unimplemented!()
    }

    fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

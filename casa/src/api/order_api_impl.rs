use api;
use api::utils;
use db;
use db::PostgresHelper;
use domain;
use hyper::mime::Mime;
use iron;
use repositories;
use repository;
use repository::RepoRead;
use repository::Retrievable;
use serde_json;
use services;

pub struct OrderApiImpl<C: PostgresHelper> {
    order_repo: repositories::OrderRepository<C>,
    order_service: services::OrderService<C>,
    settlement_service: services::SettlementService<C>,
    session_repo: repositories::SessionRepository<C>,
    user_repo: repositories::UserRepository<C>,
    db_helper: C,
}

impl<C: PostgresHelper> OrderApiImpl<C> {
    pub fn new(db_helper: C) -> Self {
        let eth_path = "/Users/mate/work/cambio/eth_test/data/geth.ipc";
        let settlement_service = services::SettlementService::new(db_helper.clone(), eth_path);
        Self {
            order_repo: repositories::OrderRepository::new(db_helper.clone()),
            order_service: services::OrderService::new(db_helper.clone()),
            settlement_service: settlement_service,
            session_repo: repositories::SessionRepository::new(db_helper.clone()),
            user_repo: repositories::UserRepository::new(db_helper.clone()),
            db_helper: db_helper.clone(),
        }
    }

    fn check_owner(
        &mut self,
        owner_id: domain::OwnerId,
        session_token: &domain::SessionToken,
    ) -> Result<(), api::ApiError> {
        let session = try!(session_token.get(&mut self.db_helper));
        if !session.is_valid() {
            return Err(api::ApiError::new(
                "You are not logged in.".to_owned(),
                api::ErrorType::NotLoggedIn,
            ));
        }
        let user = self.user_repo.get_owner(owner_id).unwrap();
        if session.email_address.unwrap() != user.email_address {
            return Err(api::ApiError::new(
                "Cannot retrieve object.".to_owned(),
                api::ErrorType::NotFound,
            ));
        }
        Ok(())
    }

    fn create_order(
        &mut self,
        order: api::OrderRequest,
        email_address: &str,
    ) -> Result<domain::Order, iron::Response> {
        let sell_currency = domain::Currency::new(order.sell_asset_type, order.sell_asset_denom);
        let buy_currency = domain::Currency::new(order.buy_asset_type, order.buy_asset_denom);
        let order_result = self.order_service.place_order(
            email_address,
            &order.unique_id,
            order.sell_asset_units,
            sell_currency,
            order.buy_asset_units,
            buy_currency,
        );

        match order_result {
            Ok(result) => Ok(result),
            err => Err(utils::to_response(err)),
        }
    }

    fn get_session(&mut self, request: &iron::Request) -> Result<domain::Session, api::ApiError> {
        let unauth_err = Err(api::ApiError::from(db::CambioError::unauthorised()));
        let session_token = match utils::get_session_token(request) {
            Some(s) => s,
            None => {
                return unauth_err;
            }
        };
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = match self.session_repo.read(&clause).map(|mut s| s.pop()) {
            Ok(Some(s)) => {
                if !s.is_valid() {
                    return Err(api::ApiError::unauthorised());
                }
                s
            }
            Ok(None) => return Err(api::ApiError::unauthorised()),
            Err(err) => return Err(api::ApiError::from(err)),
        };
        Ok(session)
    }
}

impl<C: PostgresHelper> api::OrderApiTrait for api::OrderApiImpl<C> {
    fn get_active_orders(&mut self, request: &iron::Request) -> iron::Response {
        let unauth_response = api::ApiError::unauthorised().into();
        let session_token = utils::get_session_token(request).unwrap();
        let session_clause = repository::UserClause::SessionToken(session_token);
        let session_result = self.session_repo.read(&session_clause);

        let session = match session_result.map(|mut s| s.pop()) {
            Ok(Some(session)) => session,
            Ok(None) => return unauth_response,
            Err(err) => return api::ApiError::from(err).into(),
        };

        if !session.is_valid() {
            return unauth_response;
        }
        let order_clause = repository::UserClause::All(false);
        let order_result = self.order_repo.read(&order_clause);
        match order_result {
            Ok(orders) => utils::to_response(Ok(orders)),
            err => utils::to_response(err),
        }
    }

    fn get_user_orders(&mut self, request: &iron::Request) -> iron::Response {
        let session_token = match utils::get_session_token(request) {
            Some(s) => s,
            None => {
                let unauth_err = Err(db::CambioError::unauthorised());
                return utils::to_response::<domain::Order>(unauth_err);
            }
        };
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let email_address = match self.session_repo.read(&clause).map(|mut s| s.pop()) {
            Ok(Some(s)) => s.email_address.unwrap(),
            Ok(None) => return api::ApiError::unauthorised().into(),
            Err(err) => return api::ApiError::from(err).into(),
        };
        let order_clause = repository::UserClause::EmailAddress(email_address);
        match self.order_repo.read(&order_clause) {
            Ok(orders) => utils::to_response(Ok(orders)),
            Err(err) => api::ApiError::from(err).into(),
        }
    }

    fn post_new_order(&mut self, request: &mut iron::Request) -> iron::Response {
        let unauth_resp = api::ApiError::from(db::CambioError::unauthorised());
        let order: api::OrderRequest = match api::get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };
        let email_address = match self.get_session(request).map(|s| s.email_address) {
            Ok(Some(e)) => e,
            Ok(None) => return unauth_resp.into(),
            Err(err) => return err.into(),
        };
        match self.create_order(order, &email_address) {
            Ok(order) => utils::to_response(Ok(order)),
            Err(err_resp) => return err_resp,
        }
    }

    fn post_buy_order(&mut self, request: &mut iron::Request) -> iron::Response {
        let unauth_resp = api::ApiError::from(db::CambioError::unauthorised());
        let order: api::OrderBuy = match api::get_api_obj(request) {
            Ok(obj) => obj,
            Err(response) => return response,
        };

        // locate the target order
        let order_clause = repository::UserClause::Id(order.order_id);
        let read_result = self.order_repo.read(&order_clause);
        let target_order = match read_result.map(|mut o| o.pop()) {
            Ok(Some(o)) => o,
            Ok(None) => return api::ApiError::not_found("Order").into(),
            Err(err) => return api::ApiError::from(err).into(),
        };

        // check the orders are valid and compatible with each other
        let unfair_err = db::CambioError::unfair_operation(
            "The order you chose is either incompatible or no longer active",
            "Target order.is_fair() returned false",
        );
        if !target_order.is_active() {
            let err = db::CambioError::not_permitted(
                "The order you chose is expired or no longer active",
                "Target order is expired or is not active",
            );
            return api::ApiError::from(err).into();
        }
        let request_copy = order.order_request.clone();
        let buy_currency =
            domain::Currency::new(request_copy.buy_asset_type, request_copy.buy_asset_denom);
        let sell_currency =
            domain::Currency::new(request_copy.sell_asset_type, request_copy.sell_asset_denom);
        if !target_order.is_fair(
            &buy_currency,
            &sell_currency,
            request_copy.buy_asset_units,
            request_copy.sell_asset_units,
        ) {
            return api::ApiError::from(unfair_err).into();
        }
        if !target_order.can_exchange(&buy_currency, &sell_currency) {
            return api::ApiError::from(unfair_err).into();
        }

        // create and save a corresponding order
        let session = match self.get_session(request) {
            Ok(s) => s,
            Err(err) => return err.into(),
        };
        let email_address = session.email_address.unwrap();
        let our_order = match self.create_order(order.order_request, &email_address) {
            Ok(o) => o,
            Err(resp) => return resp,
        };

        // save the settlement
        let settlement_result = if target_order.buy_asset_type.is_crypto() {
            // target_order is the buying order
            self.settlement_service
                .create_settlement(session.user_id, &target_order, &our_order)
        } else {
            self.settlement_service
                .create_settlement(session.user_id, &our_order, &target_order)
        };

        // generate the receipt
        match settlement_result {
            Ok(settlement) => {
                let response_json = serde_json::to_string(&settlement).unwrap();
                let content_type = "application/json".parse::<Mime>().unwrap();
                iron::Response::with((iron::status::Ok, response_json, content_type))
            }
            Err(err) => api::ApiError::from(err).into(),
        }
    }
}

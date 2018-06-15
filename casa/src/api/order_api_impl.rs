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
use repository::Readable;
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
        let eth_path = "http://localhost:303030";
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
        order: &api::OrderRequest,
        email_address: &str,
    ) -> Result<domain::Order, iron::Response> {
        let order_result = self.order_service.place_order(
            email_address,
            &order.unique_id,
            order.sell_asset_units as u64,
            order.sell_asset_type,
            order.buy_asset_units as u64,
            order.buy_asset_type,
            order.max_wei
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
    fn get_active_orders(&mut self) -> iron::Response {
        let order_clause = repository::UserClause::All(false);
        let order_result = self.order_repo.read(&order_clause);
        match order_result {
            Ok(orders) => utils::to_response(Ok(orders)),
            err => utils::to_response(err),
        }
    }

    fn get_user_orders(&mut self, user: &domain::User) -> iron::Response {
        let order_clause = repository::UserClause::EmailAddress(user.email_address.clone());
        match self.order_repo.read(&order_clause) {
            Ok(orders) => utils::to_response(Ok(orders)),
            Err(err) => api::ApiError::from(err).into(),
        }
    }

    fn post_new_order(&mut self, user: &domain::User, order: &api::OrderRequest) -> iron::Response {
        let unauth_resp = api::ApiError::from(db::CambioError::unauthorised());
        if order.sell_asset_type.is_crypto() && order.max_wei.is_none() {
            const WEI_MSG: &'static str = "To sell Ethereum, please specify your transaction cost";
            return api::ApiError::missing_field_or_param(WEI_MSG).into();
        }
        match self.create_order(&order, &user.email_address) {
            Ok(order) => utils::to_response(Ok(order)),
            Err(err_resp) => return err_resp,
        }
    }

    fn post_buy_order(&mut self, user: &domain::User, order: &api::OrderBuy) -> iron::Response {
        let email_address = &user.email_address;
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

        if !(target_order.sell_asset_type == request_copy.buy_asset_type && 
           target_order.buy_asset_type == request_copy.sell_asset_type && 
           target_order.sell_asset_units == request_copy.buy_asset_units && 
           target_order.buy_asset_units == request_copy.sell_asset_units) {
            return api::ApiError::from(unfair_err).into();
        }

        let our_order = match self.create_order(&order.order_request, &email_address) {
            Ok(o) => o,
            Err(resp) => return resp,
        };

        // save the settlement
        let settlement_result = if target_order.buy_asset_type.is_crypto() {
            // target_order is the buying order
            self.settlement_service
                .create_settlement(user.id.unwrap(), &target_order, &our_order)
        } else {
            self.settlement_service
                .create_settlement(user.id.unwrap(), &our_order, &target_order)
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

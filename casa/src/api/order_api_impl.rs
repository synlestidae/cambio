use api::utils;
use api;
use db;
use db::PostgresHelper;
use domain;
use iron;
use repositories;
use repository;
use repository::RepoRead;
use services;

pub struct OrderApiImpl<C: PostgresHelper>  {
    order_repo: repositories::AccountRepository<C>,
    order_service: services::OrderService<C>,
    session_repo: repositories::SessionRepository<C>,
    user_repo: repositories::UserRepository<C>
}

impl<C: PostgresHelper> OrderApiImpl<C> {
    pub fn new() -> Self {
        unimplemented!()
    }

    fn check_owner(&mut self, owner_id: domain::Id, session_token: &str) -> Result<(), api::ApiError> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = self.session_repo.read(&clause).unwrap().pop().unwrap();
        if !session.is_valid() {
            return Err(api::ApiError::new("You are not logged in.".to_owned(), api::ErrorType::NotLoggedIn));
        }
        let user = self.user_repo.get_owner(owner_id).unwrap();
        if session.email_address.unwrap() != user.email_address {
            return Err(api::ApiError::new("Cannot retrieve object.".to_owned(), api::ErrorType::NotFound));
        }
        Ok(())
    }

    fn create_order(&mut self, order: api::OrderRequest, email_address: &str) 
        -> Result<domain::Order, iron::Response> {
        let sell_currency = domain::Currency::new(order.sell_asset_type, order.sell_asset_denom);
        let buy_currency = domain::Currency::new(order.buy_asset_type, order.buy_asset_denom);
        let order_result = self.order_service.place_order(email_address, 
            &order.unique_id,
            order.sell_asset_units,
            sell_currency,
            order.buy_asset_units,
            buy_currency);

        match order_result {
            Ok(result) => Ok(result),
            err => Err(utils::to_response(err))
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
            Ok(Some(s)) => s,
            Ok(None) => return Err(api::ApiError::unauthorised()),
            Err(err) => return Err(api::ApiError::from(err))
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
            Err(err) => return api::ApiError::from(err).into()
        };

        if !session.is_valid() {
            return unauth_response;
        }
        let order_clause = repository::UserClause::All(true);
        let order_result = self.order_repo.read(&order_clause);
        match order_result {
            Ok(orders) => utils::to_response(Ok(orders)),
            err => utils::to_response(err)
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
            Err(err) => return api::ApiError::from(err).into()
        };
        let order_clause = repository::UserClause::EmailAddress(email_address);
        match self.order_repo.read(&order_clause) {
            Ok(orders) => utils::to_response(Ok(orders)),
            Err(err) => api::ApiError::from(err).into()
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
            Err(err) => return err.into()
        };
        match self.create_order(order, &email_address) {
            Ok(order) => utils::to_response(Ok(order)),
            Err(err_resp) => return err_resp
        }
    }

    fn post_buy_order(&mut self, request: &mut iron::Request) -> iron::Response {
        // locate the target order
        // check that owner is different
        // create a corresponding order
        // save that order
        // settle the order 
        // make some kind of receipt
        unimplemented!()
    }
}

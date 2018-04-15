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
}

impl<C: PostgresHelper> api::OrderApiTrait for api::OrderApiImpl<C> {
    fn get_active_orders(&mut self, request: &iron::Request) -> iron::Response {
        let clause = repository::UserClause::All(true);
        let session_token = utils::get_session_token(request);
        let session = match self.session_repo.read(&clause).map(|mut s| s.pop()) {
            Ok(Some(session)) => session, 
            Ok(None) => return api::ApiError::unauthorised().into(),
            Err(err) => return api::ApiError::from(err).into()
        };
        //utils::to_response(Ok(orders)),
        unimplemented!()
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
        let session_token = match self.session_repo.read(&clause).map(|mut s| s.pop()) {
            Ok(Some(s)) => s.email_address.unwrap(),
            Ok(None) => return api::ApiError::unauthorised().into(),
            Err(err) => return api::ApiError::from(err).into()
        };
        let order_clause = repository::UserClause::EmailAddress(session_token);
        match self.order_repo.read(&order_clause) {
            Ok(orders) => utils::to_response(Ok(orders)),
            Err(err) => api::ApiError::from(err).into()
        }
    }

    fn post_new_order(&mut self, request: &iron::Request) -> iron::Response {
        unimplemented!()
    }
    fn post_buy_order(&mut self, request: &iron::Request) -> iron::Response {
        unimplemented!()
    }
}

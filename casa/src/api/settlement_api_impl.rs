use api::SettlementApiTrait;
use iron;
use repositories;
use services;
use db::PostgresHelper;

pub struct SettlementApiImpl<C: PostgresHelper> {
    order_repo: repositories::OrderRepository<C>,
    order_service: services::OrderService<C>,
    settlement_service: services::SettlementService<C>,
    session_repo: repositories::SessionRepository<C>,
    user_repo: repositories::UserRepository<C>
}

impl<C: PostgresHelper> SettlementApiTrait for SettlementApiImpl<C> {
    fn post_complete_order(&mut self, request: &iron::Request) -> iron::Response {
        unimplemented!()
    }

    fn get_settlement_status(&mut self, request: &iron::Request) -> iron::Response {
        unimplemented!()
    }
}

use repositories;
use repository;
use services;
use db::PostgresHelper;
use api::OrderApiTrait;

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
}

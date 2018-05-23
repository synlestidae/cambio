use api::api_init::ApiInit;
use api::{AccountApiImpl, AccountApiTrait, ApiError};
use db::PostgresHelper;
use iron;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;
use router::Router;
use serde_json;
use std::borrow::Borrow;
use std::sync::Arc;

#[derive(Clone)]
pub struct AccountApiInit<T: PostgresHelper> {
    helper: T,
}

impl<T: PostgresHelper> AccountApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self { helper: helper }
    }
}

impl<T: PostgresHelper> ApiInit for AccountApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        let accounts_helper: Arc<T> = Arc::new(self.helper.clone());
        let account_helper: Arc<T> = Arc::new(self.helper.clone());

        let account_helper: Arc<T> = Arc::new(self.helper.clone());
        let accounts_helpers: Arc<T> = Arc::new(self.helper.clone());
        let transactions_helper: Arc<T> = Arc::new(self.helper.clone());
        let transaction_helper: Arc<T> = Arc::new(self.helper.clone());

        router.get(
            "/accounts/",
            move |r: &mut Request| {
                let this_helper_ref: &T = account_helper.borrow();
                let mut api = AccountApiImpl::new(this_helper_ref.clone());
                Ok(api.get_accounts(r))
            },
            "get_account",
        );

        router.get(
            "/accounts/:account_id",
            move |r: &mut Request| {
                let this_helper_ref: &T = accounts_helper.borrow();
                let mut api = AccountApiImpl::new(this_helper_ref.clone());
                Ok(api.get_account(r))
            },
            "get_accounts",
        );

        router.get(
            "/accounts/:account_id/transactions/",
            move |r: &mut Request| {
                let this_helper_ref: &T = transactions_helper.borrow();
                let mut api = AccountApiImpl::new(this_helper_ref.clone());
                Ok(api.get_transactions(r))
            },
            "get_transactions",
        );

        router.get(
            "/accounts/:account_id/transactions/:transaction_id",
            move |r: &mut Request| {
                let this_helper_ref: &T = transaction_helper.borrow();
                let mut api = AccountApiImpl::new(this_helper_ref.clone());
                Ok(api.get_transaction(r))
            },
            "get_transaction",
        );
    }
}

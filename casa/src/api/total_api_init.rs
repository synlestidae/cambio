use api::api_init::ApiInit;
use api::user_api_init::UserApiInit;
use api::account_api_init::AccountApiInit;
use db::PostgresHelper;
use router::Router;

pub struct TotalApiInit<T: PostgresHelper> {
    user_api_init: UserApiInit<T>,
    account_api_init: AccountApiInit<T>
}

impl<T: PostgresHelper> TotalApiInit<T> {
    pub fn new(helper: T, web3_address: &str) -> Self {
        Self { 
            account_api_init: AccountApiInit::new(helper.clone()),
            user_api_init: UserApiInit::new(helper, web3_address)
        }
    }
}

impl<T: PostgresHelper> ApiInit for TotalApiInit<T>
where
    T: 'static,
{
    fn init_api(&mut self, router: &mut Router) {
        self.user_api_init.init_api(router);
        self.account_api_init.init_api(router);
    }
}

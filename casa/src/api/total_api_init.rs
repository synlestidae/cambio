use api::account_api_init::AccountApiInit;
use api::api_init::ApiInit;
use api::order_api_init::OrderApiInit;
use api::payment_api_init::PaymentApiInit;
use api::settlement_api_init::SettlementApiInit;
use api::user_api_init::UserApiInit;
use db::PostgresHelper;
use router::Router;

pub struct TotalApiInit<T: PostgresHelper> {
    user_api_init: UserApiInit<T>,
    account_api_init: AccountApiInit<T>,
    payment_api_init: PaymentApiInit<T>,
    order_api_init: OrderApiInit<T>,
}

impl<T: PostgresHelper> TotalApiInit<T> {
    pub fn new(helper: T, web3_address: &str) -> Self {
        Self {
            account_api_init: AccountApiInit::new(helper.clone()),
            user_api_init: UserApiInit::new(helper.clone(), web3_address),
            payment_api_init: PaymentApiInit::new(helper.clone()),
            order_api_init: OrderApiInit::new(helper),
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
        self.payment_api_init.init_api(router);
        self.order_api_init.init_api(router);
    }
}

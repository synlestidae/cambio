use api::api_init::ApiInit;
use api::user_api_init::UserApiInit;
use db::PostgresHelper;
use router::Router;

pub struct TotalApiInit<T: PostgresHelper> {
    user_api_init: UserApiInit<T>
}

impl<T: PostgresHelper> TotalApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self {
            user_api_init: UserApiInit::new(helper)
        }
    }
}

impl<T: PostgresHelper> ApiInit for TotalApiInit<T> 
    where T: 'static {
    fn init_api(&mut self, router: &mut Router) {
        self.user_api_init.init_api(router);
    }
}

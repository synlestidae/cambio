use api::{Registration, Profile, ApiResult, ApiError, LogIn};
use domain::{User, Session};

pub trait UserApiTrait {
    fn put_register(&mut self, registration: Registration) -> ApiResult<User>;
    fn post_log_in(&mut self, log_in: LogIn) -> ApiResult<Session>;
    fn get_profile(&mut self, email_address: &str) -> ApiResult<Profile>;
    //fn complete_registration();
    //fn post_profile(&mut self);
    //fn get_bank_accounts();
    //fn post_bank_accounts();
}

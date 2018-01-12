use api::{Registration, Profile, ApiResult, ApiError, LogIn};

pub trait UserApiTrait {
    fn put_register(&mut self, registration: Registration) -> ApiResult<Registration>;
    fn post_log_in(&mut self, log_in: LogIn) -> ApiResult<LogIn>;
    fn get_profile(&mut self) -> ApiResult<Profile>;
    //fn complete_registration();
    //fn post_profile(&mut self);
    //fn get_bank_accounts();
    //fn post_bank_accounts();
}

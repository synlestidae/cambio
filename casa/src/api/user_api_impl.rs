use api::{Registration, Profile, ApiResult, ApiError, LogIn, UserApiTrait};
use db::{ConnectionSource, UserRepository, PostgresHelper};
use domain::{User, Session};

pub struct UserApi<C: PostgresHelper> {
    user_repository: UserRepository<C>
}

impl<C: PostgresHelper> UserApi<C> {
    pub fn new(helper: C) -> Self {
        Self {
            user_repository: UserRepository::new(helper)
        }
    }
}

impl<C: PostgresHelper> UserApiTrait for UserApi<C> {
    fn put_register(&mut self, registration: Registration) -> ApiResult<User> {
        // test password requirements
        if registration.password.len() < 8 {
            return Err(ApiError::bad_format("Password needs to be at least 8 characters"));
        }

        let register_result = self.user_repository.register_user(&registration.email_address,
            registration.password);

        const GENERIC_FAIL_MSG: &str = "Failed to register user";

        if let Err(error) = register_result {
            return Err(ApiError::unknown(GENERIC_FAIL_MSG));
        } else if let Ok(Some(user)) = register_result {
            return Ok(user)
        } else {
            return Err(ApiError::unknown(GENERIC_FAIL_MSG));
        }
    }

    fn post_log_in(&mut self, log_in: LogIn) -> ApiResult<Session> {
        let log_in_result = self.user_repository.log_user_in(&log_in.email_address, log_in.password);
        match log_in_result {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err(ApiError::invalid_login("User account does not exist")),
            Err(error) => Err(ApiError::unknown("Could not log you in"))
        }
    }

    fn get_profile(&mut self, email_address: &str) -> ApiResult<Profile> {
        unimplemented!()
    }
}

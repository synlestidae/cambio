use api::{
    get_api_obj, ApiError, ApiResult, ErrorType, LogIn, Registration, PersonalDetails
};
use db::{ConnectionSource, PostgresHelper, CambioError};
use domain::{Session, User, Registration as PendingRegistration};
use hyper::mime::Mime;
use iron;
use iron::headers::SetCookie;
use iron::{Request, Response};
use serde_json;
use services::UserService;
use domain;
use api;
use repository::{Creatable, Readable};

pub struct UserApi<C: PostgresHelper + Clone> {
    db: C,
    user_service: UserService<C>
}

impl<C: PostgresHelper + Clone> UserApi<C> {
    pub fn new(helper: C, web3_address: &str) -> Self {
        Self {
            db: helper.clone(),
            user_service: UserService::new(helper, web3_address),
        }
    }

    pub fn put_register(&mut self, registration: &api::Registration) -> Response {
        // test password requirements
        if registration.password.len() < 8 {
            return ApiError::bad_format("Password needs to be at least 8 characters").into();
        }

        let pending_registration = 
            PendingRegistration::new(&registration.email_address, &registration.password);

        let created_reg = match pending_registration.create(&mut self.db) {
            Ok(r) => r,
            Err(err) => return err.into()
        };

        let result = api::RegistrationInfo {
            email_address: created_reg.email_address,
            identifier_code: created_reg.identifier_code
        };

        let content_type = "application/json".parse::<Mime>().unwrap();
        let content = serde_json::to_string(&result).unwrap();
        iron::Response::with((iron::status::Ok, content, content_type))
    }

    pub fn post_confirm_register(&mut self, registration_confirm: &api::RegistrationConfirm) 
        -> Response {
            let registration = match registration_confirm.identifier_code.get(&mut self.db) {
                Ok(r) => r,
                Err(err) => return err.into()
            };
            if registration_confirm.can_confirm(&registration) {
                let register_result = self.user_service.create_user(
                    &registration.email_address, 
                    &registration.password_hash,
                    &registration_confirm.personal_details);
                match register_result {
                    Ok(user) => {
                        let content_type = "application/json".parse::<Mime>().unwrap();
                        let content = serde_json::to_string(&user).unwrap();
                        iron::Response::with((iron::status::Ok, content, content_type))
                    },
                    Err(err) => {
                        err.into()
                    }
                }
            } else {
                CambioError::unauthorised().into()
            }
    }

    pub fn post_log_in(&mut self, login: &api::LogIn) -> Response {
        let log_in_result = self
            .user_service
            .log_user_in(&login.email_address, login.password.clone());

        match log_in_result {
            Ok(result) => {
                let response_json = serde_json::to_string(&result).unwrap();
                let content_type = "application/json".parse::<Mime>().unwrap();
                let mut response =
                    iron::Response::with((iron::status::Ok, response_json, content_type));
                response.headers.set(SetCookie(vec![format!(
                    "session_token={}; Domain=localhost",
                    result.session_token
                )]));
                response
            }
            Err(cambio_err) => {
                cambio_err.into()
            }
        }
    }

    pub fn get_profile(&mut self, user: &User) -> Response {
        unimplemented!()
    }
}

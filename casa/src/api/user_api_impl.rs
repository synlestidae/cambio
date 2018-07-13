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
use postgres::GenericConnection;

pub struct UserApi<C: GenericConnection> {
    db: C,
    web3_address: String
}

impl<C: GenericConnection> UserApi<C> {
    pub fn new(db: C, web3_address: &str) -> Self {
        Self {
            db: db,
            web3_address: web3_address.to_owned()
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

        println!("Confirmation code for {} is {}", 
            created_reg.email_address, 
            created_reg.confirmation_code
        );

        let result = api::RegistrationInfo {
            email_address: created_reg.email_address,
            identifier_code: created_reg.identifier_code
        };

        let content_type = "application/json".parse::<Mime>().unwrap();
        let content = serde_json::to_string(&result).unwrap();
        iron::Response::with((iron::status::Ok, content, content_type))
    }


    pub fn post_resend_email(&mut self, 
        registration_confirm: &api::ResendEmail) -> Response {
        unimplemented!()
    }

    pub fn post_confirm_register(&mut self, registration_confirm: &api::RegistrationConfirm) 
        -> Response {
        let user_service = UserService::new("http://localhost:8081");
        let registration = match registration_confirm.identifier_code.get(&mut self.db) {
            Ok(r) => r,
            Err(err) => return err.into()
        };
        info!("Confirming registration");
        if registration_confirm.can_confirm(&registration) {
            info!("Registration with ID {} is confirmed", registration_confirm.identifier_code.0);
            let registration_result = 
                user_service.confirm_registration(
                    &mut self.db,
                    &registration, 
                    &registration_confirm.personal_details,
                    &registration_confirm.eth_account_password);
            match registration_result {
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
        let user_service = UserService::new(&self.web3_address);
        let log_in_result = user_service.log_user_in(
            &mut self.db,
            &login.email_address, 
            login.password.clone()
        );

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
                info!("Log in error {:?}", cambio_err);
                cambio_err.into()
            }
        }
    }

    pub fn get_profile(&mut self, user: &User) -> Response {
        unimplemented!()
    }
}

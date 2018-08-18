use api;
use api::{ApiError, ApiResult, ErrorType, LogIn, PersonalDetails, Registration};
use config::EmailConfig;
use db::{CambioError, ConnectionSource, PostgresHelper};
use domain;
use domain::{Registration as PendingRegistration, Session, User};
use hyper::mime::Mime;
use iron;
use iron::headers::SetCookie;
use iron::{Request, Response};
use postgres::GenericConnection;
use repository::{Creatable, Readable, Updateable};
use serde_json;
use services::UserService;
use lettre::EmailAddress;
use std::sync::mpsc::Sender;
use event::Bus;

pub struct UserApi<C: GenericConnection> {
    db: C,
    bus: Bus
}

impl<C: GenericConnection> UserApi<C> {
    pub fn new(
        db: C,
        bus: Bus
    ) -> Self {
        Self {
            db: db,
            bus: bus
        }
    }

    pub fn put_register(&mut self, registration: &api::Registration) -> Response {
        let existing_user_match = match registration.email_address.get_option(&mut self.db) {
            Ok(op) => op,
            Err(err) => return err.into()
        };
        if let Some(existing_user) = existing_user_match {
            let entity_name = &format!("User with email {}", existing_user.email_address);
            return ApiError::already_exists(entity_name).into();
        }
        // test password requirements
        if registration.password.len() < 8 {
            return ApiError::bad_format("Password needs to be at least 8 characters").into();
        }

        let pending_registration =
            PendingRegistration::new(&registration.email_address, &registration.password);

        let created_reg = match pending_registration.create(&mut self.db) {
            Ok(r) => r,
            Err(err) => return err.into(),
        };
        let email_address = created_reg.email_address;
        let result = api::RegistrationInfo {
            email_address: email_address.clone(),
            identifier_code: created_reg.identifier_code,
        };

        let content_type = "application/json".parse::<Mime>().unwrap();
        let content = serde_json::to_string(&result).unwrap();
        iron::Response::with((iron::status::Ok, content, content_type))
    }

    pub fn post_resend_email(&mut self, registration_confirm: &api::ResendEmail) -> Response {
        // TODO Does not work yet
        let registration_result = registration_confirm.identifier_code.get(&mut self.db);
        let reg: domain::Registration = match registration_result {
            Ok(reg) => reg,
            Err(err) => return err.into(),
        };
        let result = api::RegistrationInfo {
            email_address: reg.email_address,
            identifier_code: reg.identifier_code,
        };
        let content_type = "application/json".parse::<Mime>().unwrap();
        let content = serde_json::to_string(&result).unwrap();
        iron::Response::with((iron::status::Ok, content, content_type))
    }

    pub fn post_confirm_register(
        &mut self,
        registration_confirm: &api::RegistrationConfirm,
    ) -> Response {
        info!("Confirming registration");
        let user_service = UserService::new(self.bus.clone());

        let registration = match registration_confirm.identifier_code.get(&mut self.db) {
            Ok(r) => r,
            Err(err) => return err.into(),
        };
        if registration_confirm.can_confirm(&registration) {
            info!(
                "Registration with ID {} is confirmed",
                registration_confirm.identifier_code.0
            );
            let registration_result = user_service.confirm_registration(
                &mut self.db,
                &registration,
                &registration_confirm.personal_details,
                &registration_confirm.eth_account_password,
            );
            match registration_result {
                Ok(user) => {
                    let content_type = "application/json".parse::<Mime>().unwrap();
                    let content = serde_json::to_string(&user).unwrap();
                    iron::Response::with((iron::status::Ok, content, content_type))
                }
                Err(err) => err.into(),
            }
        } else {
            CambioError::unauthorised().into()
        }
    }

    pub fn post_log_in(&mut self, login: &api::LogIn) -> Response {
        let user_service = UserService::new(self.bus.clone());
        let log_in_result =
            user_service.log_user_in(&mut self.db, &login.email_address, login.password.clone());

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

    pub fn update_personal_details(&mut self, user: &User, personal_details: &PersonalDetails) 
        -> Result<api::PersonalDetails, CambioError> {
        let user_id = user.id.unwrap();
        let current_profile: domain::Profile = user_id.get(&mut self.db)?;
        let mut new_profile = personal_details.clone().into_profile(user_id); 
        new_profile.id = current_profile.id;
        Ok(api::PersonalDetails::from_profile(user_id, new_profile.update(&mut self.db)?))
    }

    pub fn get_profile(&mut self, user: &User) -> Result<api::PersonalDetails, CambioError> {
        let user_id = user.id.unwrap();
        let profile: domain::Profile = user_id.get(&mut self.db)?;
        Ok(api::PersonalDetails::from_profile(user_id, profile))
    }
}

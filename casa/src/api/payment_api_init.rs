use api::api_init::ApiInit;
use api::get_api_obj;
use api;
use repository;
use repository::{RepoRead, RepoCreate};
use db::{PostgresHelper};
use domain::{Payment, UserPayment};
use iron::{Request, Response};
use repositories::{UserPaymentRepository, SessionRepository};
use router::Router;
use std::borrow::Borrow;
use std::sync::Arc;
use api::utils::to_response;

pub struct PaymentApiInit<T: PostgresHelper> {
    helper: T
}

impl<T: PostgresHelper> PaymentApiInit<T> {
    pub fn new(helper: T) -> Self {
        Self {
            helper: helper
        }
    }
}

impl<T: 'static + PostgresHelper> ApiInit for PaymentApiInit<T> {
    fn init_api(&mut self, router: &mut Router) {
        let payment_helper: Arc<T> = Arc::new(self.helper.clone());

        router.post(
            "/payment",
            move |r: &mut Request| {
                // init the services
                let this_helper_ref: &T = payment_helper.borrow();
                let mut payment_repo = UserPaymentRepository::new(this_helper_ref.clone());
                let mut session_repo = SessionRepository::new(this_helper_ref.clone());
                
                // read the input data
                let session_token = match api::utils::get_session_token(r) {
                    Some(token) => token,
                    None => return Ok(api::ApiError::unauthorised().into())
                };
                let payment: Payment = match get_api_obj(r) {
                    Ok(payment) => payment,
                    Err(err_response) => return Ok(err_response)
                };

                // get the email address
                let clause = repository::UserClause::SessionToken(session_token.clone());
                let email_address = match session_repo.read(&clause).map(|mut s| s.pop()) {
                    Ok(Some(session)) => session.email_address.unwrap(),
                    _ => return Ok(api::ApiError::unauthorised().into())
                };
                let payment_result = payment_repo.create(&UserPayment {
                    email_address: email_address,
                    payment: payment
                });
                match payment_result {
                    Ok(payment_made) => Ok(to_response(Ok(payment_made))),
                    Err(payment_err) => {
                        let api_err: api::ApiError = payment_err.into();
                        return Ok(api_err.into())
                    }
                }
            },
            "post_payment"
        );
    }
}

use db;
use api;
use domain;
use iron::prelude::*;
use iron::middleware::Handler;
use iron::IronResult;
use api::ApiRequest;
use std::convert::TryFrom;
use api::*;
use repository::Readable;
use jobs::JobRequest;
use std::sync::mpsc::Sender;
use std::sync::Mutex;

pub struct ApiHandler<T: db::PostgresHelper> {
    db: T,
    web3_address: String,
    job_tx: Mutex<Sender<JobRequest>>
}

impl<T: db::PostgresHelper + 'static> ApiHandler<T> {
    pub fn new(db: T, web3_address: &str, job_tx: Sender<JobRequest>) -> Self {
        Self {
            db: db,
            web3_address: web3_address.to_owned(),
            job_tx: Mutex::new(job_tx)
        }
    }
}

impl<T: db::PostgresHelper + 'static + Clone + Send + Sync> Handler for ApiHandler<T> {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        let mut db = self.db.clone();
        let fake_user = domain::User { 
            id: None, 
            email_address: "".to_owned(),
            password: None,
            password_hash: None,
            owner_id: None
        };
        let user: domain::User = match request.get_session_token() {
            Some(token) => {
                match token.get_option(&mut db) {
                    Ok(Some(user)) => {
                        match user.user_id.get(&mut db) {
                            Ok(user) => user,
                            Err(err) => return Ok(err.into())
                        }
                    },
                    Ok(None) => fake_user,
                    Err(err) => return Ok(err.into())
                }
            },
            None => {
                fake_user
                // just make a fake user that will never get used
            }
        };
        let api_request_result: Result<ApiRequest, _> = TryFrom::try_from(request);
        let api_request = match api_request_result {
            Ok(r) => r,
            Err(err) => return Ok(err.into())
        };
        if api_request.requires_auth() && user.id.is_none() {
            return Ok(api::ApiError::unauthorised().into())
        }
        let response = match api_request {
            ApiRequest::User(user_request) => {
                let mut user_api = UserApi::new(db.clone(), &self.web3_address);
                match user_request {
                    UserRequest::Register(reg) => user_api.put_register(&reg),
                    UserRequest::LogIn(login) => user_api.post_log_in(&login)
                }
            },
            ApiRequest::Order(order_request) => {
                let mut order_api = OrderApiImpl::new(db); 
                match order_request {
                    OrderApiRequest::GetActiveOrders => order_api.get_active_orders(),
                    OrderApiRequest::GetUserOrders => order_api.get_user_orders(&user), 
                    OrderApiRequest::PostNewOrder(new_order) => order_api.post_new_order(&user, &new_order),
                    OrderApiRequest::PostBuyOrder(order_buy) => order_api.post_buy_order(&user, &order_buy),
                }
            },
            ApiRequest::Account(account_request) => {
                let mut account_api = AccountApiImpl::new(db); 
                match account_request {
                    AccountRequest::GetAccounts => account_api.get_accounts(&user),
                    AccountRequest::GetAccount(account_id) => account_api.get_account(&user, account_id),
                    AccountRequest::GetAccountTransactions(account_id) => {
                        account_api.get_transactions(&user, account_id)
                    },
                    AccountRequest::GetAccountTransaction(account_id, transaction_id) => {
                        account_api.get_transaction(&user, account_id, transaction_id)
                    },
                }
            },
            ApiRequest::Settlement(settlement_request) => {
                match settlement_request {
                    SettlementRequest::PostSettlementEthAuth(order_id, cred) => {
                        unimplemented!()
                    },
                    _ => unimplemented!()
                }
            },
            ApiRequest::Payment(payment_req) => {
                match payment_req {
                    PaymentRequest::CreditCardPayment(payment) => unimplemented!()
                }
            }
        };

        Ok(response)
    }
}

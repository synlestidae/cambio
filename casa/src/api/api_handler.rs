use api;
use api::ApiRequest;
use api::*;
use db;
use domain;
use iron::middleware::Handler;
use iron::prelude::*;
use iron::IronResult;
use jobs::JobRequest;
use postgres::GenericConnection;
use postgres::{Connection, TlsMode};
use repository::Readable;
use repository::Updateable;
use std::convert::TryFrom;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use web3;

pub struct ApiHandler {
    conn_str: String,
    job_tx: Mutex<Sender<JobRequest>>,
    web3: web3::Web3<web3::transports::ipc::Ipc>,
    eloop: web3::transports::EventLoopHandle
}

impl ApiHandler {
    pub fn new(conn_str: &str, web3_address: &str, job_tx: Sender<JobRequest>) -> Self {
        let (eloop, transport) = web3::transports::ipc::Ipc::new(web3_address).unwrap();
        Self {
            conn_str: conn_str.to_owned(),
            job_tx: Mutex::new(job_tx),
            eloop: eloop,    
            web3: web3::Web3::new(transport)
        }
    }
}

impl Handler for ApiHandler {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        let conn_str: &str = &self.conn_str;
        let mut db = Connection::connect(conn_str, TlsMode::None).unwrap();
        let fake_user = domain::User {
            id: None,
            email_address: "".to_owned(),
            password: None,
            password_hash: None,
            owner_id: None,
        };
        let user: domain::User = match request.get_session_token() {
            Some(token) => {
                let session: Result<Option<domain::Session>, db::CambioError> =
                    token.get_option(&mut db);
                if let Ok(Some(mut s)) = session {
                    s.renew();
                    match s.update(&mut db) {
                        Err(err) => println!("Failed to renew session: {:?}", err),
                        _ => (),
                    }
                }
                match token.get_option(&mut db) {
                    Ok(Some(user)) => match user.user_id.get(&mut db) {
                        Ok(user) => user,
                        Err(err) => return Ok(err.into()),
                    },
                    Ok(None) => fake_user,
                    Err(err) => return Ok(err.into()),
                }
            }
            None => {
                fake_user
                // just make a fake user that will never get used
            }
        };
        let api_request_result: Result<ApiRequest, _> = TryFrom::try_from(request);
        let api_request = match api_request_result {
            Ok(r) => r,
            Err(err) => return Ok(err.into()),
        };
        if api_request.requires_auth() && user.id.is_none() {
            return Ok(api::ApiError::unauthorised().into());
        }

        let response = match api_request {
            ApiRequest::User(user_request) => {
                let mut user_api = UserApi::new(db, self.web3.clone());
                match user_request {
                    UserRequest::Register(reg) => user_api.put_register(&reg),
                    UserRequest::ResendEmail(email_resend) => {
                        user_api.post_resend_email(&email_resend)
                    }
                    UserRequest::Confirm(confirm) => user_api.post_confirm_register(&confirm),
                    UserRequest::LogIn(login) => user_api.post_log_in(&login),
                }
            }
            ApiRequest::Order(order_request) => {
                let mut order_api = OrderApiImpl::new(db);
                match order_request {
                    OrderApiRequest::GetActiveOrders => order_api.get_active_orders(),
                    OrderApiRequest::GetUserOrders => order_api.get_user_orders(&user),
                    OrderApiRequest::PostNewOrder(new_order) => {
                        order_api.post_new_order(&user, &new_order)
                    }
                    OrderApiRequest::PostBuyOrder(order_buy) => {
                        order_api.post_buy_order(&user, &order_buy)
                    }
                }
            }
            ApiRequest::Account(account_request) => {
                let mut account_api = AccountApiImpl::new(db);
                match account_request {
                    AccountRequest::GetAccounts => account_api.get_accounts(&user),
                    AccountRequest::GetAccount(account_id) => {
                        account_api.get_account(&user, account_id)
                    }
                    AccountRequest::GetAccountTransactions(account_id) => {
                        account_api.get_transactions(&user, account_id)
                    }
                    AccountRequest::GetAccountTransaction(account_id, transaction_id) => {
                        account_api.get_transaction(&user, account_id, transaction_id)
                    }
                }
            }
            ApiRequest::Settlement(settlement_request) => {
                let tx = self.job_tx.lock().unwrap();
                let mut settlement_api = SettlementApiImpl::new(db, tx.clone());
                match settlement_request {
                    SettlementRequest::PostSettlementEthAuth(order_id, cred) => {
                        settlement_api.post_settlement_eth_auth(&user, order_id, &cred)
                    }
                    _ => unimplemented!(),
                }
            }
            ApiRequest::Payment(payment_req) => unimplemented!(),
        };

        Ok(response)
    }
}

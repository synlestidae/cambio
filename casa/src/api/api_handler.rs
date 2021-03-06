use api;
use api::ApiRequest;
use api::*;
use config::ServerConfig;
use db;
use domain;
use iron::middleware::Handler;
use iron::prelude::*;
use iron::IronResult;
use postgres::GenericConnection;
use postgres::{Connection, TlsMode};
use repository::Readable;
use colectivo::Colectivo;
use repository::Updateable;
use std::convert::TryFrom;
use std::sync::mpsc::Sender;
use std::sync::Mutex;
use event::Bus;
use colectivo::Topic;

pub struct ApiHandler {
    server_config: ServerConfig,
    colectivo: Colectivo
}

impl ApiHandler {
    pub fn new(
        server_config: &ServerConfig,
        colectivo: Colectivo
    ) -> Self {
        Self {
            server_config: server_config.clone(),
            colectivo: colectivo
        }
    }

    fn get_bus(&self, topic: Topic) -> Bus {
        let (p, c) = self.colectivo.channel(topic);
        Bus::new(p, c)
    }
}

impl Handler for ApiHandler {
    fn handle<'a, 'b>(&self, request: &mut Request<'a, 'b>) -> IronResult<Response> {
        let conn_str: &str = &self.server_config.get_connection_string();
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
                        Err(err) => error!("Failed to renew session: {:?}", err),
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
                let mut user_api = UserApi::new(db, self.get_bus("registration".into()));
                match user_request {
                    UserRequest::Register(reg) => user_api.put_register(&reg),
                    UserRequest::ResendEmail(email_resend) => {
                        user_api.post_resend_email(&email_resend)
                    }
                    UserRequest::Confirm(confirm) => user_api.post_confirm_register(&confirm),
                    UserRequest::LogIn(login) => user_api.post_log_in(&login),
                    profile_request => {
                        let result = match profile_request {
                            UserRequest::SetPersonalDetails(ref details) => user_api.update_personal_details(&user, details),
                            UserRequest::GetPersonalDetails => user_api.get_profile(&user),
                            _ => unreachable!(),
                        };
                        match result {
                            Err(err) => err.into(),
                            ok_result => api::utils::to_response(ok_result),
                        }
                    }
                }
            }
            ApiRequest::Order(order_request) => {
                let mut order_api = OrderApiImpl::new(db);
                match order_request {
                    OrderApiRequest::GetActiveOrders => order_api.get_active_orders(),
                    OrderApiRequest::GetChangedOrders(last_change) => {
                        match order_api.get_changed_orders(&last_change.last_change) {
                            Err(err) => err.into(),
                            success => api::utils::to_response(success)
                        }
                    },
                    OrderApiRequest::GetUserOrders => order_api.get_user_orders(&user),
                    OrderApiRequest::PostNewOrder(new_order) => {
                        match order_api.post_new_order(&user, &new_order) {
                            Err(err) => err.into(),
                            ok_resp => api::utils::to_response(ok_resp),
                        }
                    }
                    OrderApiRequest::PostBuyOrder(completion_request) => {
                        match order_api.complete_sell_order(&user, &completion_request) {
                            Err(err) => err.into(),
                            ok_resp => api::utils::to_response(ok_resp),
                        }
                    }
                    OrderApiRequest::PostSellOrder(completion_request) => {
                        match order_api.complete_buy_order(&user, &completion_request) {
                            Err(err) => err.into(),
                            ok_resp => api::utils::to_response(ok_resp)
                        }
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
                    },
                    AccountRequest::GetAccountStatement(account_id) => {
                        api::utils::to_response(account_api.get_statement(&user, account_id))
                    }
                }
            }
            ApiRequest::Settlement(settlement_request) => {
                let mut settlement_api = SettlementApiImpl::new(db);
                api::utils::to_response(match settlement_request {
                    SettlementRequest::GetUserSettlements => settlement_api.get_user_settlements(&user)
                })
            },
            ApiRequest::Payment(payment_req) => {
                let mut payment_api = PaymentApi::new(&self.server_config.get_poli_config(), db);
                match payment_api.request_payment(&user, &payment_req) {
                    Err(err) => err.into(),
                    Ok(payment_response) => api::utils::to_response(Ok(payment_response)),
                }
            },
            ApiRequest::CryptoAccount(crypto_api_request) => {
                let mut crypto_account_api = CryptoAccountApi::new(db);
                match crypto_api_request {
                    CryptoAccountApiRequest::GetAccounts => {
                        api::utils::to_response(crypto_account_api.get_accounts(&user))
                    },
                    CryptoAccountApiRequest::NewAccount(new_account) => api::utils::to_response(crypto_account_api.new_account(&user, &new_account)),
                    CryptoAccountApiRequest::ModifyAccount(existing_account) => api::utils::to_response(crypto_account_api.edit_account(&user, &existing_account)),
                }
            }
        };

        Ok(response)
    }
}

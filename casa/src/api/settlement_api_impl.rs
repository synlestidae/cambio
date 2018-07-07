use api;
use api::SessionTokenSource;
use db;
use db::PostgresHelper;
use db::TransactionSource;
use domain;
use iron;
use repositories;
use db::TryFromRow;
use repository::RepoUpdate;
use query::Selectable;
use repository::Readable;
use services;
use web3::types::U256;
use std::sync::mpsc::Sender;
use jobs::JobRequest;
use db::ConnectionSource;
use db::Transaction;
use bcrypt::{verify};

pub struct SettlementApiImpl<C: PostgresHelper + ConnectionSource> {
    db: C,
    job_tx: Sender<JobRequest>
}

impl<H: PostgresHelper + ConnectionSource> SettlementApiImpl<H> {
    pub fn new(db: H, job_tx: Sender<JobRequest>) -> Self {
        Self {
            db: db,
            job_tx: job_tx
        }
    }
    
    pub fn post_settlement_eth_auth(&mut self, 
        user: &domain::User, 
        order_id: domain::OrderId, 
        credentials: &api::SettlementEthCredentials) -> iron::Response {

        let tx = self.db.get().unwrap();
        {
            let transaction = tx.transaction().unwrap();
            let db = db::PostgresTransactionHelper::new(transaction);
            info!("Processing credentials for order {:?}", order_id); 
            let order: domain::Order = match order_id.get(&mut self.db) {
                Ok(o) => o,
                Err(err) => {
                    info!("Order {:?} does not exist", order_id);
                    return err.into();
                }
            };

            if Some(order.owner_id) != user.owner_id {
                info!("Order {:?} has owner {:?}, but expected {:?}", order_id, order.owner_id, user.owner_id);
                let err = db::CambioError::not_found_search("That order does not exist", 
                    "User trying to access another's order");
                return err.into();
            }

            // retrieve the settlement
            info!("Checking settlement exists");
            let mut settlement: domain::OrderSettlement = match order_id.get(&mut self.db) {
                Ok(s) => s,
                Err(err) => return err.into()
            };

            info!("Checking settlement status is correct");

            if settlement.settlement_status != domain::SettlementStatus::WaitingEthCredentials {
                let sys_msg =
                    format!("Expected settlement status to be WaitingEthCredentials, got {:?}", 
                        settlement.settlement_status);
                return db::CambioError::not_permitted(
                    "Settlement is not waiting for credentials", 
                    &sys_msg).into()
            }

            info!("Getting Ethereum account");

            // TODO make this unwrap unnecessary
            let eth_account: domain::EthAccount = match user.owner_id.unwrap().get(&mut self.db) {
                Ok(e) => e,
                Err(err) => return err.into()
            };
            info!("Checking the password user used!");
            let pwd_result = verify(&credentials.password, &eth_account.password_hash_bcrypt);
            if let Ok(true) = pwd_result {
                info!("Sending settlement to the job queue");
                let req = JobRequest::BeginSettlement(settlement.id.unwrap(), credentials.password.to_owned());
                match self.job_tx.send(req) {
                    Ok(s) => {
                        info!("Job was placed on queue");
                        db.commit();
                        iron::response::Response::with((iron::status::Status::Ok, format!("")))
                    }, 
                    Err(_) => {
                        info!("Job queue was unavailable");
                        let send_err = db::CambioError::shouldnt_happen(
                            "Could not get your settlement on the blockchain.", 
                            "Channel to job loop is disconnected.");
                        send_err.into()
                    }
                }
            } else {
                db::CambioError::invalid_password().into()
            }
        }
    }

    pub fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

use api;
use api::SessionTokenSource;
use bcrypt::verify;
use db;
use db::ConnectionSource;
use db::PostgresHelper;
use db::Transaction;
use db::TransactionSource;
use db::TryFromRow;
use domain;
use iron;
use jobs::JobRequest;
use postgres::GenericConnection;
use query::Selectable;
use repositories;
use repository::Readable;
use repository::RepoUpdate;
use services;
use std::sync::mpsc::Sender;
use web3::types::U256;

pub struct SettlementApiImpl<C: GenericConnection> {
    job_tx: Sender<JobRequest>,
    db: C,
}

impl<C: GenericConnection> SettlementApiImpl<C> {
    pub fn new(db: C, job_tx: Sender<JobRequest>) -> Self {
        Self {
            db: db,
            job_tx: job_tx,
        }
    }

    pub fn post_settlement_eth_auth(
        &mut self,
        user: &domain::User,
        order_id: domain::OrderId,
        credentials: &api::SettlementEthCredentials,
    ) -> iron::Response {
        let mut transaction = self.db.transaction().unwrap();
        info!("Processing credentials for order {:?}", order_id);
        let order: domain::Order = match order_id.get(&mut transaction) {
            Ok(o) => o,
            Err(err) => {
                info!("Order {:?} does not exist", order_id);
                return err.into();
            }
        };

        if Some(order.owner_id) != user.owner_id {
            info!(
                "Order {:?} has owner {:?}, but expected {:?}",
                order_id, order.owner_id, user.owner_id
            );
            let err = db::CambioError::not_found_search(
                "That order does not exist",
                "User trying to access another's order",
            );
            return err.into();
        }

        // retrieve the settlement
        info!("Checking settlement exists");
        let mut settlement: domain::OrderSettlement = match order_id.get(&mut transaction) {
            Ok(s) => s,
            Err(err) => return err.into(),
        };

        info!("Checking settlement status is correct");

        if settlement.settlement_status != domain::SettlementStatus::WaitingEthCredentials {
            let sys_msg = format!(
                "Expected settlement status to be WaitingEthCredentials, got {:?}",
                settlement.settlement_status
            );
            return db::CambioError::not_permitted(
                "Settlement is not waiting for credentials",
                &sys_msg,
            ).into();
        }

        info!("Getting Ethereum account");

        // TODO make this unwrap unnecessary
        let eth_account: domain::EthAccount = match user.owner_id.unwrap().get(&mut transaction) {
            Ok(e) => e,
            Err(err) => return err.into(),
        };
        info!("Checking the password user used!");
        let pwd_result = verify(&credentials.password, &eth_account.password_hash_bcrypt);
        if let Ok(true) = pwd_result {
            info!("Sending settlement to the job queue");
            let req = JobRequest::BeginSettlement(
                settlement.id.unwrap(),
                credentials.password.to_owned(),
            );
            match self.job_tx.send(req) {
                Ok(s) => {
                    info!("Job was placed on queue");
                    transaction.commit();
                    iron::response::Response::with((iron::status::Status::Ok, format!("")))
                }
                Err(_) => {
                    info!("Job queue was unavailable");
                    let send_err = db::CambioError::shouldnt_happen(
                        "Could not get your settlement on the blockchain.",
                        "Channel to job loop is disconnected.",
                    );
                    send_err.into()
                }
            }
        } else {
            db::CambioError::invalid_password().into()
        }
    }

    pub fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

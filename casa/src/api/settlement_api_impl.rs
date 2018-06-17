use api;
use api::SessionTokenSource;
use db;
use db::PostgresHelper;
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
        let conn = match self.db.get() {
            Ok(c) => c,
            Err(err) => { 
                let err: db::CambioError = err.into();
                return err.into()
            }
        };
        let tx = match conn.transaction() {
            Ok(tx) => tx,
            Err(err) => {
                let err: db::CambioError = err.into();
                return err.into()
            }
        };
        let mut tx_helper = db::PostgresTransactionHelper::new(tx);
        let order: domain::Order = match order_id.get(&mut tx_helper) {
            Ok(o) => o,
            Err(err) => return err.into()
        };

        if Some(order.owner_id) != user.owner_id {
            let err = db::CambioError::not_found_search("That order does not exist", 
                "User trying to access another's order");
            return err.into();
        }

        // retrieve the settlement
        let mut settlement: domain::OrderSettlement = match order_id.get(&mut tx_helper) {
            Ok(s) => s,
            Err(err) => return err.into()
        };
        if settlement.settlement_status != domain::SettlementStatus::WaitingEthCredentials {
            return db::CambioError::not_permitted(
                "Settlement is not waiting for credentials", 
                "Settlement status is not WaitingEthCredentials").into()
        }
        let eth_account: domain::EthAccount = match self.owner_id.get(&mut tx_helper) {
            Ok(e) => e,
            Err(err) => return err.into()
        };
        let pwd_result = verify(&credentials.password, eth_account.password_hash_crypt);
        if let Ok(true) != pwd_result {
            return CambioError::invalid_password().into()
        }
        let req = JobRequest::BeginSettlement(settlement.id.unwrap(), credentials.password.to_owned());
        self.job_tx.send(req).unwrap();
        tx_helper.commit();
        iron::response::Response::with((iron::status::Status::Ok, format!("")))
    }

    pub fn get_settlement_status(&mut self, request: &mut iron::Request) -> iron::Response {
        unimplemented!()
    }
}

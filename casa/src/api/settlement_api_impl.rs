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
use hyper::mime::Mime;
use iron;
use jobs::JobRequest;
use postgres::GenericConnection;
use repository::Readable;
use repository::RepoUpdate;
use serde_json;
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
        unimplemented!()
    }

    pub fn get_settlement_status(&mut self, order_id: &domain::OrderId) -> iron::Response {
        let settlement: domain::OrderSettlement = match order_id.get(&mut self.db) {
            Ok(settlement) => settlement,
            Err(err) => return err.into(),
        };
        let content_type = "application/json".parse::<Mime>().unwrap();
        let content = serde_json::to_string(&settlement).unwrap();
        iron::Response::with((iron::status::Ok, content, content_type))
    }
}

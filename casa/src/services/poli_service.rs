use payment::poli::{PoliConfig, InitiateTransaction, InitiateTransactionResponse};
use db::{PostgresHelper, Transaction};
use domain::{User, PoliPaymentRequest};
use db::CambioError;
use hyper::client::Client;

pub struct PoliService<H: PostgresHelper + Transaction> {
    poli_config: PoliConfig,
    db: H
}

impl<'a, T: PostgresHelper + Transaction> PoliService<T> {
    pub fn new(poli_config: PoliConfig, db: T) -> Self {
        Self {
            poli_config: poli_config, 
            db: db
        }
    }

    pub fn initiate_transaction(&mut self, poli_payment_request: &PoliPaymentRequest) 
        -> Result<InitiateTransactionResponse, CambioError> {
        let init_tx = InitiateTransaction::from_request(
            &self.poli_config,
            poli_payment_request
        );
        unimplemented!()
    }
}

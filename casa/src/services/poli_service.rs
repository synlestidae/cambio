use payment::poli::{PoliConfig, InitiateTransaction};
use db::{PostgresHelper, Transaction};
use domain::{User};
use api::PaymentRequest;
use db::CambioError;

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

    pub fn initiate_transaction(&mut self, payment_request: &PaymentRequest) 
        -> Result<InitiateTransaction, CambioError> {
        unimplemented!()
    }
}

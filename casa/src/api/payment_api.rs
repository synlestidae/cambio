use db::*;
use db::Transaction;
use domain::*;
use payment::poli::*;
use api::{PaymentRequest, RequestPaymentResponse};
use services::{PoliService, PoliError};
use chrono::prelude::*;
use iron::response::Response;
use repository::{Readable, Creatable, Updateable};
use iron;

pub struct PaymentApi<H: ConnectionSource> {
    conn_src: H,
    poli_config: PoliConfig
}

impl<H: ConnectionSource> PaymentApi<H> {
    pub fn new(poli_config: PoliConfig, conn_src: H) -> Self {
        Self {
            conn_src: conn_src,
            poli_config: poli_config 
        }
    }

    pub fn request_payment(&mut self, 
        user: &User,
        payment: &PaymentRequest) -> Result<RequestPaymentResponse, CambioError> {
        let conn = try!(self.conn_src.get());
        let user_id = user.id.clone().unwrap();
        let mut tx = PostgresTransactionHelper::new(try!(conn.transaction()));
        let mut poli_service = self.get_poli_service();
        let mut payment_req = PoliPaymentRequest::new(user_id, payment.amount);
        payment_req = try!(payment_req.create(&mut tx));
        let mut tx_response = match poli_service.initiate_transaction(&payment_req) {
            Ok(tx_response) => tx_response,
            Err(err) => {
                return Err(err.into())
            }
        };
        let resp = match tx_response.get_transaction() {
            Ok(poli_tx) => {
                payment_req.transaction_token = Some(poli_tx.transaction_token);
                payment_req.payment_status = PaymentStatus::StartedWithPoli;
                try!(payment_req.update(&mut tx));
                RequestPaymentResponse {
                    navigate_url: poli_tx.navigate_url
                }
            },
            Err(err) => {
                let poli_err = PoliError::from(err);
                self.save_in_log(&user.id, &poli_err);
                payment_req.payment_status = PaymentStatus::Failed;
                try!(payment_req.update(&mut tx));
                return Err(poli_err.into())
            }
        };
        tx.commit();
        Ok(resp)
    }

    pub fn handle_nudge(&mut self, nudge: &Nudge) 
        -> Result<RequestPaymentResponse, CambioError> {
        let conn = try!(self.conn_src.get());
        let mut db = PostgresTransactionHelper::new(try!(conn.transaction()));
        let poli_service = self.get_poli_service();
        let tx_result = poli_service.get_transaction(&nudge.token);
        let tx = match tx_result {
            Ok(tx) => tx,
            Err(err) => {
                self.save_in_log(&None, &err);
                return Err(err.into());
            }
        };
        let poli_payment = try!(nudge.transaction_token.read(&mut db));
        unimplemented!()
    }

    fn get_poli_service(&self) -> PoliService {
         PoliService::new(
            &self.poli_config
        )
    }

    fn save_in_log(&mut self, user_id: &Option<UserId>, err: &PoliError) {
        let mut log_conn = match self.conn_src.get() {
            Ok(c) => c,
            Err(_) => return
        };
        drop(err.save_in_log(user_id, &mut log_conn));
    }
}

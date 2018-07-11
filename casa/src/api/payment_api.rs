use db::*;
use db::Transaction;
use domain::*;
use payment::poli::*;
use api::{PaymentRequest, RequestPaymentResponse};
use services::{PoliService};
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
        let mut log_conn = try!(self.conn_src.get());
        let conn_tx = try!(conn.transaction());
        let mut tx = PostgresTransactionHelper::new(conn_tx);
        let mut poli_service = PoliService::new(
            self.poli_config.clone()
        );
        let user_id = user.id.clone().unwrap();
        let mut payment_req = PoliPaymentRequest {
            id: None,
            user_id: user_id.clone(),
            amount: payment.amount.clone(),
            unique_code: Code::new(),
            started_at: Utc::now(),
            payment_status: PaymentStatus::StartedByUser,
            transaction_token: None
        };
        payment_req = try!(payment_req.create(&mut tx));
        let mut tx_response = match poli_service.initiate_transaction(&payment_req) {
            Ok(tx_response) => tx_response,
            Err(err) => {
                err.save_in_log(user_id, &mut log_conn);
                return Err(err.into())
            }
        };
        let resp = match tx_response.transaction.pop() {
            Some(poli_tx) => {
                payment_req.transaction_token = Some(poli_tx.transaction_token);
                payment_req.payment_status = PaymentStatus::StartedWithPoli;
                try!(payment_req.update(&mut tx));
                RequestPaymentResponse {
                    navigate_url: poli_tx.navigate_url
                }
            },
            None => {
                payment_req.payment_status = PaymentStatus::Failed;
                try!(payment_req.update(&mut tx));
                unimplemented!()
            }
        };
        tx.commit();
        Ok(resp)
    }
}

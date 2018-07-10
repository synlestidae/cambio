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
        let conn = self.conn_src.get().unwrap();
        let conn_tx = conn.transaction();
        let mut tx = PostgresTransactionHelper::new(conn_tx.unwrap());
        let mut poli_service = PoliService::new(
            self.poli_config.clone()
        );
        let mut payment_req = PoliPaymentRequest {
            id: None,
            user_id: user.id.unwrap(),
            amount: payment.amount.clone(),
            unique_code: Code::new(),
            started_at: Utc::now(),
            payment_status: PaymentStatus::StartedByUser,
            transaction_token: None
        };
        payment_req.create(&mut tx).unwrap();
        let mut tx_response = poli_service.initiate_transaction(&payment_req).unwrap();
        let resp = match tx_response.transaction.pop() {
            Some(poli_tx) => {
                payment_req.transaction_token = Some(poli_tx.transaction_token);
                payment_req.payment_status = PaymentStatus::StartedWithPoli;
                payment_req.update(&mut tx).unwrap();
                RequestPaymentResponse {
                    navigate_url: poli_tx.navigate_url
                }
            },
            None => {
                payment_req.payment_status = PaymentStatus::Failed;
                payment_req.update(&mut tx).unwrap();
                unimplemented!()
            }
        };
        tx.commit();
        Ok(resp)
    }
}

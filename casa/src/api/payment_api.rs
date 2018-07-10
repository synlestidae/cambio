use db::*;
use db::Transaction;
use domain::*;
use payment::poli::*;
use api::PaymentRequest;
use services::{PoliService};
use chrono::prelude::*;
use iron::response::Response;
use repository::{Readable, Creatable};
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
        payment: &PaymentRequest) -> Response {
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
        let tx_response = poli_service.initiate_transaction(&payment_req).unwrap();
        payment_req.transaction_token = Some(tx_response.transaction.unwrap().transaction_token);
        tx.commit();
        unimplemented!()
    }
}

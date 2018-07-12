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
use services::LedgerService;

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

        // Retrieve the transaction from our DB and Poli 
        let tx_result = poli_service.get_transaction(&nudge.token);
        let poli_tx = match tx_result {
            Ok(tx) => tx,
            Err(err) => {
                self.save_in_log(&None, &err);
                return Err(err.into());
            }
        };
        let payment_request = try!(nudge.token.get(&mut db));
        let user: User = try!(payment_request.user_id.get(&mut db));
        let owner_id = user.owner_id.unwrap();
        let account_set = try!(AccountSet::from(try!(owner_id.get_vec(&mut db))));

        // requirements for credit
        // * PaymentRequest not marked as completed
        // * PaymentRequest is marked as StartedWithPoli
        // * TransactionResponse has no errors
        // * TransactionResponse currency matches credit account currency

        if let Some(err) = poli_tx.error_code {
            unimplemented!()
        }

        if payment_request.payment_status != PaymentStatus::StartedWithPoli {
            unimplemented!()
        }

        let mut conn2 = try!(self.conn_src.get());
        let mut ledger_service = LedgerService::new(&mut conn2); 

        let poli_deduct_account = unimplemented!();
        let user_wallet_account = account_set.nzd_wallet();

        // account may now be credit
        ledger_service.transfer_money(poli_deduct_account, user_wallet_account);
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

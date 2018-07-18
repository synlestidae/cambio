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
use postgres::GenericConnection;

pub struct PaymentApi<C: GenericConnection> {
    poli_config: PoliConfig,
    db: C
}

impl<C: GenericConnection> PaymentApi<C> {
    pub fn new(poli_config: PoliConfig, db: C) -> Self {
        Self {
            poli_config: poli_config, 
            db: db
        }
    }

    pub fn request_payment(&mut self, 
        user: &User,
        payment: &PaymentRequest) -> Result<RequestPaymentResponse, CambioError> {
        let mut tx = try!(self.db.transaction());
        let user_id = user.id.clone().unwrap();
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
                payment_req.transaction_ref_no = Some(poli_tx.transaction_ref_no);
                payment_req.payment_status = PaymentStatus::StartedWithPoli;
                try!(payment_req.update(&mut tx));
                RequestPaymentResponse {
                    navigate_url: poli_tx.navigate_url
                }
            },
            Err(err) => {
                let poli_err = PoliError::from(err);
                //self.save_in_log(&mut tx, &user.id, &poli_err);
                payment_req.payment_status = PaymentStatus::Failed;
                try!(payment_req.update(&mut tx));
                return Err(poli_err.into())
            }
        };
        tx.commit();
        Ok(resp)
    }

    pub fn handle_nudge(&mut self, db: &mut C, nudge: &Nudge) 
        -> Result<RequestPaymentResponse, CambioError> {
        let mut conn = try!(self.db.transaction());
        let poli_service = self.get_poli_service();

        // Retrieve the transaction from our DB and Poli 
        let poli_tx_result = poli_service.get_transaction(&nudge.token);
        let poli_tx = match poli_tx_result {
            Ok(tx) => tx,
            Err(err) => {
                //self.save_in_log(&mut conn, &None, &err);
                return Err(err.into());
            }
        };
        let payment_request = try!(nudge.token.get(&mut conn));
        let user: User = try!(payment_request.user_id.get(&mut conn));
        let owner_id = user.owner_id.unwrap();
        let account_set = try!(AccountSet::from(try!(owner_id.get_vec(&mut conn))));

        // requirements for credit
        // * PaymentRequest not marked as completed
        // * PaymentRequest is marked as StartedWithPoli
        // * TransactionResponse has no errors
        // * TransactionResponse currency matches credit account currency

        if let Some(err) = poli_tx.error_code {
            //conn.commit();
            //err.save_in_log(None, conn.connection());
            //return Err(err.into());
            unimplemented!()
        }

        if payment_request.payment_status != PaymentStatus::StartedWithPoli {
            return Err(CambioError::not_permitted(
                "This payment cannot be nudged.", 
                &format!("Expected payment to be in state `StartedWithPoli`, but got `{:?}`", payment_request.payment_status))
            );
        }

        let mut ledger_service = LedgerService::new(); 
        let poli_deduct_account: AccountId = try!(PaymentVendor::Poli.get(db));
        let user_wallet_account = account_set.nzd_wallet();

        // check deduct account has role System and business type SystemFeesPaid 
        // check credited account has role Primary and business type UserCashWallet 

        // account may now be credited
        try!(ledger_service.transfer_money(db, 
            poli_deduct_account, 
            user_wallet_account, 
            poli_tx.amount_paid)
        );
        unimplemented!()
    }

    fn get_poli_service(&self) -> PoliService {
         PoliService::new(
            &self.poli_config
        )
    }

    fn save_in_log(&mut self, db: &mut C, user_id: &Option<UserId>, err: &PoliError) {
        drop(err.save_in_log(user_id, db));
    }
}

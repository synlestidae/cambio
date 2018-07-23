use api::{PaymentRequest, RequestPaymentResponse};
use chrono::prelude::*;
use db::Transaction;
use db::*;
use domain::*;
use iron;
use iron::response::Response;
use payment::poli::*;
use postgres::GenericConnection;
use repository::{Creatable, Readable, Updateable};
use services::LedgerService;
use services::{PoliError, PoliService};

pub struct PaymentApi<C: GenericConnection> {
    poli_config: PoliConfig,
    db: C,
}

impl<C: GenericConnection> PaymentApi<C> {
    pub fn new(poli_config: PoliConfig, db: C) -> Self {
        Self {
            poli_config: poli_config,
            db: db,
        }
    }

    pub fn request_payment(
        &mut self,
        user: &User,
        payment: &PaymentRequest,
    ) -> Result<RequestPaymentResponse, CambioError> {
        let mut tx = try!(self.db.transaction());
        let user_id = user.id.clone().unwrap();
        let mut poli_service = self.get_poli_service();
        let mut payment_req = PoliPaymentRequest::new(user_id, payment.amount);
        payment_req = try!(payment_req.create(&mut tx));
        let mut tx_response = match poli_service.initiate_transaction(&payment_req) {
            Ok(tx_response) => tx_response,
            Err(err) => return Err(err.into()),
        };
        let resp = match tx_response.get_transaction() {
            Ok(poli_tx) => {
                payment_req.transaction_ref_no = Some(poli_tx.transaction_ref_no);
                payment_req.payment_status = PaymentStatus::StartedWithPoli;
                try!(payment_req.update(&mut tx));
                RequestPaymentResponse {
                    navigate_url: poli_tx.navigate_url,
                }
            }
            Err(err) => {
                let poli_err = PoliError::from(err);
                //self.save_in_log(&mut tx, &user.id, &poli_err);
                payment_req.payment_status = PaymentStatus::Failed;
                try!(payment_req.update(&mut tx));
                return Err(poli_err.into());
            }
        };
        tx.commit();
        Ok(resp)
    }

    pub fn handle_nudge(&mut self, db: &mut C, nudge: &Nudge) -> Result<(), CambioError> {
        let mut conn = try!(self.db.transaction());
        let poli_service = self.get_poli_service();

        // Retrieve the transaction from our DB and Poli
        let poli_tx_result = poli_service.get_transaction(&nudge.token);
        let poli_tx =
            match poli_tx_result {
                Ok(tx) => match tx.get_transaction() {
                    Ok(tx) => tx,
                    Err(Some(err)) => {
                        return Err(err.into());
                    }
                    Err(None) => return Err(CambioError::shouldnt_happen(
                        "There was an unknown error with the Poli transation",
                        "Poli service get_transaction returned a transaction with unknown error",
                    )),
                },
                Err(err) => return Err(err.into()),
            };
        let mut payment_request = try!(nudge.token.get(&mut conn));
        let user: User = try!(payment_request.user_id.get(&mut conn));
        let owner_id = user.owner_id.unwrap();
        let account_set = try!(AccountSet::from(try!(owner_id.get_vec(&mut conn))));

        // requirements for credit
        // * PaymentRequest not marked as completed
        // * PaymentRequest is marked as StartedWithPoli
        // * TransactionResponse has no errors
        // * TransactionResponse currency matches credit account currency

        // TODO Deal with getting error from GetTransactionResponse

        if payment_request.payment_status != PaymentStatus::StartedWithPoli {
            return Err(CambioError::not_permitted(
                "This payment cannot be nudged.",
                &format!(
                    "Expected payment to be in state `StartedWithPoli`, but got `{:?}`",
                    payment_request.payment_status
                ),
            ));
        }

        let mut ledger_service = LedgerService::new();
        let poli_deduct_account_id: AccountId = try!(PaymentVendor::Poli.get(&mut conn));
        let user_wallet_account_id = account_set.nzd_wallet();

        // check deduct account has role System and business type SystemFeesPaid
        let poli_deduct_account: Account = try!(poli_deduct_account_id.get(&mut conn));
        if !poli_deduct_account.is_for_deducting_payments() {
            payment_request.payment_status = PaymentStatus::Unknown;
            try!(payment_request.update(&mut conn));
            conn.commit();
            return Err(CambioError::shouldnt_happen(
                "Error while finding account to credit.",
                &format!("Poli deduct account returned false for is_for_deducting_payments"),
            ));
        }
        // credited account already assured by nzd_wallet() logic
        // account may now be credited
        payment_request.amount_paid = payment_request.amount_paid + poli_tx.amount_paid;
        payment_request.payment_status = PaymentStatus::Completed;
        try!(payment_request.update(&mut conn));
        try!(ledger_service.transfer_money(
            &mut conn,
            poli_deduct_account_id,
            user_wallet_account_id,
            poli_tx.currency_code.asset_type(),
            poli_tx.amount_paid
        ));
        Ok(())
    }

    fn get_poli_service(&self) -> PoliService {
        PoliService::new(&self.poli_config)
    }

    fn save_in_log(&mut self, db: &mut C, user_id: &Option<UserId>, err: &PoliError) {
        drop(err.save_in_log(user_id, db));
    }
}

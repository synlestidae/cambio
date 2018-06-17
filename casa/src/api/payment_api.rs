use db::PostgresHelper;
use domain::{Payment, User, Account, AccountSet};
use iron::response::Response;
use repository::Readable;
use iron;

pub struct PaymentApi<H: PostgresHelper + 'static> {
    db: H
}

impl<H: PostgresHelper + 'static> PaymentApi<H> {
    pub fn new(db: H) -> Self {
        Self {
            db: db
        }
    }

    pub fn post_payment(&mut self, 
        user: &User,
        payment: &Payment) -> Response {
        const CREDIT_FUNC: &'static str = "
            SELECT credit_account_from_payment($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);
        ";
        let accounts: Vec<Account> = match user.owner_id.unwrap().get_vec(&mut self.db) {
            Ok(accounts) => accounts,
            Err(err) => return err.into()
        };
        let account_set: AccountSet = match AccountSet::from(accounts) {
            Ok(accounts) => accounts,
            Err(err) => return err.into()
        };
        let account_id = account_set.nzd_wallet();
        match self.db.execute(CREDIT_FUNC, &[
            &user.id,
            &user.email_address,
            &account_id,
            &payment.asset_type,
            &payment.vendor,
            &payment.payment_method,
            &payment.datetime_payment_made.naive_utc(),
            &payment.unique_id,
            &payment.user_credit,
            &"Test credit of {} from user."
        ]) {
            Ok(_) => iron::response::Response::with((iron::status::Status::Ok, format!(""))), //unimplemented!(),
            Err(err) => err.into(),
        }
    }
}

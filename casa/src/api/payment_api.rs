use db::PostgresHelper;
use domain::{Payment, User, Account};
use iron::response::Response;
use repository::Readable;

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
        // TODO this is distinctly wrong
        let account: Account = user.owner_id.unwrap().get(&mut self.db).unwrap();
        let account_id = account.id;
        match self.db.execute(CREDIT_FUNC, &[
            &user.id,
            &user.email_address,
            &account_id,
            &payment.asset_type,
            &payment.vendor,
            &payment.payment_method,
            &payment.user_credit,
            &"Test credit of {} from user."
        ]) {
            Ok(_) => unimplemented!(),
            Err(err) => err.into(),
        }
    }
}

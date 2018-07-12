use postgres::Connection;
use domain::Transaction;
use domain::AccountId;
use db::CambioError;

pub struct LedgerService<'a> {
    db: &'a mut Connection 
}

impl<'a> LedgerService<'a> {
    pub fn new(db: &'a mut Connection) -> Self {
        Self {
            db: db
        }
    }

    pub fn transfer_money(&mut self, deduct_account: AccountId, credit_account: AccountId) 
        -> Result<(Transaction, Transaction), CambioError> {
        unimplemented!()
    }
}

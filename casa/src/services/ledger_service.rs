use postgres::GenericConnection;
use db::CambioError;
use domain::Transaction;
use domain::AccountId;
use domain::Decimal;

pub struct LedgerService {
}

impl LedgerService {
    pub fn new() -> Self {//db: &'a mut Connection) -> Self {
        Self { }
    }

    pub fn transfer_money<C: GenericConnection>(&self, 
        db: &mut C, deduct_account: AccountId, credit_account: AccountId, amount: Decimal) 
        -> Result<(Transaction, Transaction), CambioError> {
        unimplemented!()
    }
}

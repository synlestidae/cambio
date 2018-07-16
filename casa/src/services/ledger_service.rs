use postgres::GenericConnection;
use db::{TryFromRow, CambioError};
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
        let rows = try!(db.query("SELECT transfer_asset('NZD', '2018-01-01', '2018-03-31', $1, $2, $3);", &[
            &deduct_account,
            &credit_account,
            &amount
        ]));
        let err = Err(CambioError::db_update_failed("Journal"));
        if rows.len() > 0 {
            let correspondence_id_option: Option<i32> = rows.get(0).get(0);
            if let Some(correspondence_id) = correspondence_id_option {
                let transactions = try!(db.query("SELECT * FROM journal WHERE correspondence_id = $1", &[
                    &correspondence_id
                ]));
                if transactions.len() == 2 {
                    let tx1: Transaction = try!(TryFromRow::try_from_row(&transactions.get(0)));
                    let tx2: Transaction = try!(TryFromRow::try_from_row(&transactions.get(1)));
                    return Ok((tx1, tx2));
                }
            }
            err
        } else {
            err
        }
    }
}

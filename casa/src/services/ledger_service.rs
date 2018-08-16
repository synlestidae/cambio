use db::{CambioError, TryFromRow};
use domain::AccountId;
use domain::AssetType;
use domain::Decimal;
use domain::Transaction;
use postgres::GenericConnection;

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> Self {
        //db: &'a mut Connection) -> Self {
        Self {}
    }

    pub fn transfer_money<C: GenericConnection>(
        &self,
        db: &mut C,
        deduct_account: AccountId,
        credit_account: AccountId,
        asset_type: AssetType,
        amount: Decimal,
    ) -> Result<Transaction, CambioError> {
        let amount_cents = amount.to_cents();
        info!("Transferring {} cents from {:?} to {:?}", amount_cents, deduct_account, credit_account);
        let rows = try!(db.query(
            "SELECT transfer_asset($1, '2018-01-01', '2018-03-31', $2, $3, $4);",
            &[&asset_type, &deduct_account, &credit_account, &amount_cents]
        ));
        let err = Err(CambioError::db_update_failed("Journal"));
        if rows.len() > 0 {
            let correspondence_id_option: Option<i32> = rows.get(0).get(0);
            if let Some(correspondence_id) = correspondence_id_option {
                let entry = try!(db.query(
                    "SELECT 
                        journal_to.correspondence_id,
                        journal_from.account_id as from_account, 
                        journal_to.account_id as to_account, 
                        journal_from.asset_type, 
                        journal_from.debit as value, 
                        journal_from.transaction_time, 
                        journal_from.accounting_period as accounting_period_id,
                        journal_to.balance as balance_to_account,
                        journal_from.balance as balance_from_account
                    FROM 
                        journal journal_from,
                        journal journal_to
                    WHERE 
                        journal_from.correspondence_id = journal_to.correspondence_id AND
                        journal_from.correspondence_id = $1 AND 
                        journal_from.debit >= 0 AND 
                        journal_to.credit >= 0",
                    &[&correspondence_id]
                ));
                if entry.len() == 1 {
                    let tx: Transaction = try!(TryFromRow::try_from_row(&entry.get(0)));
                    return Ok(tx);
                }
            }
            err
        } else {
            err
        }
    }

    pub fn transfer_money_positive_deduction<C: GenericConnection>(
        &self,
        db: &mut C,
        deduct_account: AccountId,
        credit_account: AccountId,
        asset_type: AssetType,
        amount: Decimal,
    ) -> Result<Transaction, CambioError> {
        let mut tx = db.transaction()?;
        let transaction = 
            self.transfer_money(&mut tx, deduct_account, credit_account, asset_type, amount)?;
        // TODO: double check the balance where the account is a positive balance account
        /*if transaction.balance_from_account < 0 {
            tx.set_rollback();
            return Err(CambioError::unfair_operation(
                "Insufficient funds to complete the order.", 
                "Balance of wallet too low.")
            );
        }*/

        tx.commit()?;
        Ok(transaction)
    }
}

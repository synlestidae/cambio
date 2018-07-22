use db::{CambioError, ErrorKind, ErrorReccomendation, PostgresHelper};
use domain::{Account, AccountId, AccountStatement, Transaction};
use postgres::GenericConnection;
use repository::Readable;
use std::error::Error;

pub struct AccountService {}

impl AccountService {
    pub fn new() -> AccountService {
        AccountService {}
    }

    pub fn get_latest_statement<C: GenericConnection>(
        &self,
        db: &mut C,
        account_id: AccountId,
    ) -> Result<AccountStatement, CambioError> {
        let account = try!(account_id.get(db));
        let mut transactions = try!(self.get_transactions_for_account(db, account_id));

        transactions.sort_by_key(|t: &Transaction| t.correspondence_id);

        let mut opening_balance = 0;
        let mut closing_balance = 0;

        if transactions.len() > 0 {
            opening_balance = (&transactions[0]).balance_to_account;
            closing_balance = (&transactions[transactions.len() - 1]).balance_to_account;
        }

        Ok(AccountStatement {
            account: account,
            opening_balance: opening_balance,
            closing_balance: closing_balance,
            transactions: transactions,
        })
    }

    pub fn get_transactions_for_account<C: GenericConnection>(
        &self,
        db: &mut C,
        account_id: AccountId,
    ) -> Result<Vec<Transaction>, CambioError> {
        let transactions = account_id.get_vec(db)?;
        Ok(transactions)
    }
}

const LATEST_STATEMENT_QUERY: &'static str = "
    SELECT 
        journal_to.correspondence_id,
        journal_from.account_id as from_account, 
        journal_to.account_id as to_account, 
        journal_from.asset_type, 
        journal_from.debit as value, 
        journal_from.transaction_time, 
        journal_from.accounting_period as accounting_period_id,
        journal_to.balance as balance_to_account
    FROM 
        journal journal_from,
        journal journal_to
    WHERE 
        journal_to.account_id = $1
        journal_from.correspondence_id = journal_to.correspondence_id AND
        journal_from.correspondence_id = $1 AND 
        journal_from.debit >= 0 AND 
        journal_to.credit >= 0
    ORDER BY journal_to.correspondence_id
";

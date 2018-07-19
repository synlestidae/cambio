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

        transactions.sort_by_key(|t: &Transaction| t.id);

        let mut opening_balance = 0;
        let mut closing_balance = 0;

        if transactions.len() > 0 {
            opening_balance = (&transactions[0]).balance;
            closing_balance = (&transactions[transactions.len() - 1]).balance;
        }

        Ok(AccountStatement {
            account: account,
            opening_balance: opening_balance,
            closing_balance: closing_balance,
            transactions: transactions,
        })
    }

    pub fn get_transactions_for_account(
        &self,
        db: &mut GenericConnection,
        account_id: AccountId,
    ) -> Result<Vec<Transaction>, CambioError> {
        //let transactions = try!(db.query(LATEST_STATEMENT_QUERY, &[&account_id]));
        //Ok(transactions)
        unimplemented!()
    }
}

const LATEST_STATEMENT_QUERY: &'static str = "
    SELECT *, users.id as user_id, journal.id as journal_entry_id, journal.account_id AS to_account_id, account.asset_type as asset_type
    FROM journal
        JOIN account ON journal.account_id = account.id
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id 
        JOIN accounting_period ON journal.accounting_period = accounting_period.id
        JOIN authorship ON journal.authorship_id = authorship.id
    WHERE
        journal.correspondence_id = journal.correspondence_id AND
        account.id = $1 AND
        accounting_period.id = (SELECT MAX(id) FROM accounting_period) 
    ORDER BY journal.id
";

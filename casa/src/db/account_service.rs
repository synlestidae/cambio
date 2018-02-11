use db::{PostgresHelper, CambioError, ErrorKind, ErrorReccomendation};
use std::error::Error;
use domain::{Account, Id, AccountStatement, Transaction};

const LATEST_STATEMENT_QUERY: &'static str = "
    SELECT *, users.id as user_id, journal.id as journal_entry_id FROM users
        JOIN account_owner ON users.id = account_owner.user_id
        JOIN account ON account_owner.id = account.owner_id 
        JOIN journal ON account.id = journal.account_id 
        JOIN accounting_period ON journal.accounting_period = accounting_period.id
        JOIN authorship ON journal.authorship_id = authorship.id
        JOIN asset_type ON account.asset_type = asset_type.id
    WHERE
        account.id = $1 AND
        accounting_period = (SELECT MAX(id) FROM accounting_period) 
    ORDER BY journal.id
";

const ACCOUNT_QUERY_USER: &'static str = "
    SELECT *, account.id as account_id FROM account 
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id
        JOIN asset_type ON account.asset_type = asset_type.id
    WHERE 
       users.id = $1";

const ACCOUNT_QUERY_ID: &'static str = "
    SELECT *, account.id as account_id FROM account 
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id
        JOIN asset_type ON account.asset_type = asset_type.id
    WHERE 
        account.id = $1";

#[derive(Clone)]
pub struct AccountService<T: PostgresHelper> {
    db_helper: T,
}

impl<T: PostgresHelper> AccountService<T> {
    pub fn new(db_helper: T) -> AccountService<T> {
        AccountService { db_helper: db_helper }
    }

    pub fn get_accounts_for_user(
        &mut self,
        //email_address: &str,
        user_id: Id
    ) -> Result<Vec<Account>, CambioError> {
        let accounts = try!(self.db_helper.query(ACCOUNT_QUERY_USER, &[&user_id]));
        Ok(accounts)
    }

    pub fn get_account(&mut self, account_id: Id) -> Result<Option<Account>, CambioError> {
        let mut accounts = try!(self.db_helper.query(ACCOUNT_QUERY_ID, &[&account_id])); 
        Ok(accounts.pop())
    }

    pub fn get_latest_statement(
        &mut self,
        account_id: Id,
    ) -> Result<AccountStatement, CambioError> {
        let account_match = try!(self.get_account(account_id));
        let error = CambioError::not_found_search("Your account could not be found.", 
            &format!("Account with ID {} not found", account_id));
        let account = match account_match {
            Some(acc) => acc,
            None => return Err(error)
        };

        let mut transactions = try!(self.get_transactions_for_account(account_id));

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
        &mut self,
        account_id: Id,
    ) -> Result<Vec<Transaction>, CambioError> {
        let transactions = try!(self.db_helper.query(LATEST_STATEMENT_QUERY, &[&account_id]));
        Ok(transactions)
    }
}

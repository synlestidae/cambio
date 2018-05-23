use db::{CambioError, ErrorKind, ErrorReccomendation, PostgresHelper};
use domain::{Account, AccountStatement, Id, Transaction};
use repositories;
use repository;
use repository::*;
use std::error::Error;

#[derive(Clone)]
pub struct AccountService<T: PostgresHelper> {
    account_repo: repositories::AccountRepository<T>,
    db_helper: T,
}

impl<T: PostgresHelper> AccountService<T> {
    pub fn new(db_helper: T) -> AccountService<T> {
        AccountService {
            db_helper: db_helper.clone(),
            account_repo: repositories::AccountRepository::new(db_helper),
        }
    }

    pub fn get_latest_statement(
        &mut self,
        account_id: Id,
    ) -> Result<AccountStatement, CambioError> {
        let account_match = try!(
            self.account_repo
                .read(&repository::UserClause::Id(account_id))
        ).pop();
        let error = CambioError::not_found_search(
            "Your account could not be found.",
            &format!("Account with ID {} not found", account_id),
        );
        let account = match account_match {
            Some(acc) => acc,
            None => return Err(error),
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

const LATEST_STATEMENT_QUERY: &'static str = "
    SELECT *, users.id as user_id, journal.id as journal_entry_id, journal.account_id AS to_account_id
    FROM journal
        JOIN account ON journal.account_id = account.id
        JOIN account_owner ON account.owner_id = account_owner.id
        JOIN users ON account_owner.user_id = users.id 
        JOIN accounting_period ON journal.accounting_period = accounting_period.id
        JOIN authorship ON journal.authorship_id = authorship.id
        JOIN asset_type ON account.asset_type = asset_type.id
    WHERE
        journal.correspondence_id = journal.correspondence_id AND
        account.id = $1 AND
        accounting_period.id = (SELECT MAX(id) FROM accounting_period) 
    ORDER BY journal.id
";

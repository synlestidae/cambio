use db::{PostgresHelper, PostgresHelperError, AccountRepository, UserRepository};
use std::error::Error;
use domain::{Account, Payment, AccountRole, Transaction, AccountStatement, Id};
use chrono::{DateTime, Utc};

const CALL_CREDIT_ACCOUNT_PROCEDURE: &'static str  = 
    "SELECT credit_account_from_payment($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)";

pub struct PaymentRepository<T: PostgresHelper> {
    db_helper: T,
    account_repository: AccountRepository<T>,
    user_repository: UserRepository<T>
}

impl<T: PostgresHelper> PaymentRepository<T> {
    pub fn new(db_helper: T, 
               account_repository: AccountRepository<T>, 
               user_repository: UserRepository<T>) -> PaymentRepository<T> {
        PaymentRepository {
            db_helper: db_helper,
            account_repository: account_repository,
            user_repository: user_repository
        }
    }

    pub fn register_credit_payment(&mut self, email_address: &str, payment: &Payment) 
        -> Result<AccountStatement, PostgresHelperError> {

        // get the accounts for the user 
        let account_list = try!(self.account_repository.get_accounts_for_user(email_address));
        let user = match try!(self.user_repository.get_user_by_email(email_address)) {
            Some(user) => user,
            None => return Err(PostgresHelperError::new("Could not find user with that email address"))
        };
        // extract the PRIMARY account with matching asset and denom
        let mut creditable_accounts: Vec<Account> = account_list
            .into_iter()
            .filter(|account| account.asset_type == payment.asset_type &&
                account.asset_denom == payment.asset_denom &&
                account.account_role == AccountRole::Primary)
            .collect();
        let account_not_found_error = PostgresHelperError::new("Not matching account for credit found");
        let account = try!(creditable_accounts.pop().ok_or(account_not_found_error));

        // TODO check any limits and flag them
        // e.g. a credit of $1,000,000 is certainly wrong and needs to be checked
        // or just cancelled

        let user_id = try!(user.id.ok_or(PostgresHelperError::new("User object doesn't have ID")));
        let account_id = try!(account.id.ok_or(PostgresHelperError::new("Account object doesn't have ID")));

        // call the payment stored procedure
        let procedure_result = self.db_helper.execute(CALL_CREDIT_ACCOUNT_PROCEDURE, &[
            &user_id,
            &user.email_address,    
            &account_id,
            &account.asset_type.to_string(),
            &account.asset_denom.to_string(),
            &payment.vendor.to_string(),
            &payment.payment_method,
            &payment.datetime_payment_made,
            &payment.unique_id,
            &payment.user_credit
        ]);

        let account_id = try!(account.id.ok_or(PostgresHelperError::new("Account instance has no ID")));

        match procedure_result {
            Ok(_) => self.account_repository.get_latest_statement(&account_id),
            Err(err) => {
                println!("Error crediting: {:?}", err);
                Err(PostgresHelperError::new(&format!("Failed to credit account: {}", err)))
            }
        }
    }
}

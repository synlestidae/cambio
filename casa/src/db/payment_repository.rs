use db::{PostgresHelper, CambioError, AccountRepository, UserRepository, ErrorReccomendation, ErrorKind};
use std::error::Error;
use domain::{Account, Payment, AccountRole, Transaction, AccountStatement, Id, PaymentBuilder};
use chrono::{DateTime, Utc};

const CALL_CREDIT_ACCOUNT_PROCEDURE: &'static str = "SELECT credit_account_from_payment(user_id_var := $1, 
        email_address_var := $2, 
        credited_account_id := $3, 
        asset_type_var := $4, 
        asset_denom_var := $5, 
        vendor_name := $6, 
        payment_method_var := $7, 
        datetime_payment_made_var := $8, 
        unique_id := $9, 
        units := $10,
        message_var := $11)";

pub struct PaymentRepository<T: PostgresHelper> {
    account_repository: AccountRepository<T>,
    user_repository: UserRepository<T>,
    db_helper: T
}

impl<T: PostgresHelper> PaymentRepository<T> {
    pub fn new(db_helper: T) -> PaymentRepository<T> {
        PaymentRepository {
            account_repository: AccountRepository::new(db_helper.clone()),
            user_repository: UserRepository::new(db_helper.clone()),
            db_helper: db_helper
        }
    }

    pub fn register_credit_payment(
        &mut self,
        email_address: &str,
        payment: &Payment,
    ) -> Result<AccountStatement, CambioError> {
        let q = repository::UserClause::EmailAddress(email_address.to_owned());
        let user_not_found = CambioError::not_found_search(
            "No user found with that email address", 
            "get_user_by_email returned None"
        );
        let user = try!(try!(self.user_repository.read(&q)).ok_or(user_not_found));
        let user_id: Id = user.id.unwrap();

        let account_list =
            try!(self.account_repository.get_accounts_for_user(user.id.unwrap()));
        let message = format!("Credit to wallet using {}", payment.vendor);

        // extract the PRIMARY account with matching asset and denom
        let mut creditable_accounts: Vec<Account> = account_list
            .into_iter()
            .filter(|account| {
                account.asset_type == payment.asset_type &&
                    account.asset_denom == payment.asset_denom &&
                    account.account_role == AccountRole::Primary
            })
            .collect();
        let mut account_not_found_error = CambioError::not_found_search(
            "An account to credit was not found", 
            "Failed to found account with matching asset type for payment");
        account_not_found_error.reccomendation = ErrorReccomendation::ContactSupport;
        let account = try!(creditable_accounts.pop().ok_or(account_not_found_error));

        // TODO check any limits and flag them
        // e.g. a credit of $1,000,000 is certainly wrong and needs to be checked
        // or just cancelled

        let account_id = account.id.unwrap();

        // call the payment stored procedure
        let procedure_result = self.db_helper.execute(
            CALL_CREDIT_ACCOUNT_PROCEDURE,
            &[
                &user_id,
                &user.email_address,
                &account_id,
                &account.asset_type,
                &account.asset_denom,
                &payment.vendor,
                &payment.payment_method,
                &payment.datetime_payment_made.naive_utc(),
                &payment.unique_id,
                &payment.user_credit,
                &message,
            ],
        );

        try!(procedure_result);

        // if getting account ID failed, user can work around it
        let mut account_error = CambioError::shouldnt_happen("Could not find your account while loading transactions.", 
            "Account id field was None");
        account_error.reccomendation = ErrorReccomendation::Continue;

        // load the statement
        let account_id = try!(account.id.ok_or(account_error));
        let statement = self.account_repository.get_latest_statement(account_id);
        statement
    }
}

use db;
use domain;
use domain::Id;
use postgres::types::ToSql;
use repositories;
use repository;
use repository::*;
use services;

#[derive(Clone)]
pub struct UserPaymentRepository<T: db::PostgresHelper> {
    user_repository: repositories::UserRepository<T>,
    account_service: services::AccountService<T>,
    account_repo: repositories::AccountRepository<T>,
    db_helper: T,
}

impl<T: db::PostgresHelper> UserPaymentRepository<T> {
    pub fn new(db: T) -> Self {
        UserPaymentRepository {
            user_repository: repositories::UserRepository::new(db.clone()),
            account_service: services::AccountService::new(db.clone()),
            account_repo: repositories::AccountRepository::new(db.clone()),
            db_helper: db,
        }
    }

    pub fn register_credit_payment(
        &mut self,
        email_address: &str,
        payment: &domain::Payment,
    ) -> Result<domain::AccountStatement, db::CambioError> {
        let q = repository::UserClause::EmailAddress(email_address.to_owned());
        let user_not_found = db::CambioError::not_found_search(
            "No user found with that email address",
            "get_user_by_email returned None",
        );
        let user_match = try!(self.user_repository.read(&q)).pop();
        let user = try!(user_match.ok_or(user_not_found));
        let user_id: domain::UserId = user.id.unwrap();
        let q = repository::UserClause::EmailAddress(user.email_address.clone());
        let account_list = try!(self.account_repo.read(&q));
        let message = format!("Credit to wallet using {}", payment.vendor);

        // extract the PRIMARY account with matching asset and denom
        let mut creditable_accounts: Vec<domain::Account> = account_list
            .into_iter()
            .filter(|account| {
                account.asset_type == payment.asset_type
                    && account.account_role == domain::AccountRole::Primary
            })
            .collect();
        let mut account_not_found_error = db::CambioError::not_found_search(
            "An account to credit was not found",
            "Failed to found account with matching asset type for payment",
        );
        account_not_found_error.reccomendation = db::ErrorReccomendation::ContactSupport;
        let account = try!(creditable_accounts.pop().ok_or(account_not_found_error));

        // TODO check any limits and flag them
        // e.g. a credit of $1,000,000 is certainly wrong and needs to be checked
        // or just cancelled

        let account_id = account.id.unwrap();

        // call the payment stored procedure
        let params: &[&ToSql] = &[
            &user_id,
            &user.email_address,
            &account_id,
            &account.asset_type,
            &payment.vendor,
            &payment.payment_method,
            &payment.datetime_payment_made.naive_utc(),
            &payment.unique_id,
            &payment.user_credit,
            &message,
        ];
        let procedure_result = self
            .db_helper
            .execute(CALL_CREDIT_ACCOUNT_PROCEDURE, params);

        try!(procedure_result);

        // if getting account ID failed, user can work around it
        let mut account_error = db::CambioError::shouldnt_happen(
            "Could not find your account while loading transactions.",
            "Account id field was None",
        );
        account_error.reccomendation = db::ErrorReccomendation::Continue;

        // load the statement
        let account_id = try!(account.id.ok_or(account_error));
        let statement = self.account_service.get_latest_statement(account_id);
        statement
    }
}

impl<T: db::PostgresHelper> RepoCreate for UserPaymentRepository<T> {
    type Item = domain::UserPayment;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        try!(self.register_credit_payment(&item.email_address, &item.payment));
        let mut payments: Vec<domain::Payment> = try!(
            self.db_helper
                .query(SELECT_USER_PAYMENT, &[&item.payment.unique_id])
        );

        match payments.pop() {
            Some(payment) => Ok(domain::UserPayment {
                email_address: item.email_address.to_owned(),
                payment: payment,
            }),
            _ => Err(db::CambioError::db_update_failed("UserPayment")),
        }
    }
}

const SELECT_USER_PAYMENT: &'static str = "
    SELECT 
        user_payment.id, 
        user_payment.unique_id,
        user_payment.datetime_payment_made, 
        user_payment.payment_method, 
        user_payment.asset_type,
        vendor.name as vendor, 
        user_payment.units as user_credit
    FROM user_payment 
    JOIN vendor ON user_payment.vendor = vendor.id
    WHERE user_payment.unique_id = $1
";

const CALL_CREDIT_ACCOUNT_PROCEDURE: &'static str =
    "SELECT credit_account_from_payment(
        user_id_var := $1, 
        email_address_var := $2, 
        credited_account_id := $3, 
        asset_type_var := $4, 
        vendor_name := $5, 
        payment_method_var := $6, 
        datetime_payment_made_var := $7, 
        unique_id := $8, 
        units := $9,
        message_var := $10)";

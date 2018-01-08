use db::{PostgresHelper, PostgresHelperError};
use std::error::Error;
use domain::{Account, Payment};
use chrono::{DateTime, Utc};

pub struct PaymentRepository<T: PostgresHelper> {
    db_helper: T
}

impl<T: PostgresHelper> PaymentRepository<T> {
    pub fn new(db_helper: T) -> PaymentRepository<T> {
        PaymentRepository {
            db_helper: db_helper
        }
    }

    pub fn register_credit_payment(email_address: &str, payment: &Payment) {
        // get the accounts for the user 
        // extract the PRIMARY account with matching asset and denom
        // log in the system account 
        // TODO check any limits and flag them
        // call the payment stored procedure
    }
}

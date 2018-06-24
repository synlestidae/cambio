use db::{CambioError, PostgresHelper};
use repository::Readable;
use postgres::rows::Rows;
use domain;
use std;
use postgres::types::FromSql;

pub trait Creatable where Self: std::marker::Sized {
    type Id: Readable<Self> + FromSql;
    fn create<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError> {
        let update_failed = CambioError::db_update_failed("Entity");
        let result = try!(self.run_sql(db));
        if result.is_empty() {
            return Err(update_failed)
        }
        let id: Self::Id = match result.get(0).get("id") {
                Some(id) => id,
                None => return Err(update_failed)
        };
        Ok(try!(id.get(db)))
    }
    fn run_sql<H: PostgresHelper>(&self, db: &mut H) -> Result<Rows, CambioError>;
}

impl Creatable for domain::User {
    type Id = domain::UserId;
    fn run_sql<H: PostgresHelper>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = 
            "INSERT INTO users(email_address, password_hash) VALUES ($1, $2) RETURNING id";
        Ok(try!(db.query_raw(QUERY, &[
            &self.email_address,
            &self.password_hash
        ])))
    }
}

impl Creatable for domain::EthAccount {
    type Id = domain::EthAccountId;

    fn run_sql<H: PostgresHelper>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = 
            "INSERT INTO ethereum_account_details(address, password, owner_id) 
             VALUES ($1, $2, $3) RETURNING id";
        let address = self.address.iter().map(|&x| x).collect::<Vec<u8>>();
        Ok(try!(db.query_raw(QUERY, &[
            &address, &self.password_hash_bcrypt, &self.owner_id
        ])))
    }
}

impl Creatable for domain::Registration {
    type Id = domain::RegistrationId;

    fn run_sql<H: PostgresHelper>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = "
            INSERT INTO registration(email_address, password_hash, confirmation_code, identifier_code, requested_at, confirmed_at)
            VALUES($1, $2, $3, $4, $5, $6)
            RETURNING id
        ";
        let result = try!(db.query_raw(QUERY, &[
            &self.email_address, 
            &self.password_hash, 
            &self.confirmation_code, 
            &self.identifier_code, 
            &self.requested_at, 
            &self.confirmed_at, 
        ]));
        Ok(result)
    }
}

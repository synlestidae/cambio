use db::{CambioError, PostgresHelper};
use repository::Readable;
use domain;
use std;

pub trait Updateable where Self: std::marker::Sized {
    fn update<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError>;
}

impl Updateable for domain::OrderSettlement {
    fn update<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError> {
        if let Some(id) = self.id {
            const UPDATE_SETTLEMENT: &'static str = 
                "UPDATE asset_order SET order_status = $2 WHERE id = $1;";
            try!(db.execute(UPDATE_SETTLEMENT, &[&self.id, &self.settlement_status])); 
            id.get(db)
        } else {
            Err(CambioError::format_obj("Failed to locate settlement.", "Settlement is missing ID field."))
        }
    }
}

impl Updateable for domain::Registration {
    fn update<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError> {
        let id = match self.id {
            Some(id) => id,
            None => return Err(CambioError::db_update_failed("Registration"))
        };
        const QUERY: &'static str = "
            UPDATE registration 
                SET email_address = $1,
                SET password_hash = $2,
                SET confirmation_code = $2,
                SET identifier_code = $3,
                SET requested_at = $4,
                SET confirmed_at = $5,
            WHERE id = $6
        ";
        try!(db.execute(QUERY, &[
            &self.email_address, 
            &self.password_hash, 
            &self.confirmation_code, 
            &self.identifier_code, 
            &self.requested_at, 
            &self.confirmed_at, 
            &id])
        );
        Ok(try!(id.get(db)))
    }
}

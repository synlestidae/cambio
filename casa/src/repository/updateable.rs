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

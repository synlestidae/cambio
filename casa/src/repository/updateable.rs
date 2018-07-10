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
                SET confirmation_code = $3,
                SET identifier_code = $4,
                SET requested_at = $5,
                SET confirmed_at = $6,
            WHERE id = $7
        ";
        let update_result = db.execute(QUERY, &[
            &self.email_address, 
            &self.password_hash, 
            &self.confirmation_code, 
            &self.identifier_code, 
            &self.requested_at, 
            &self.confirmed_at, 
            &id
        ]);
        Ok(try!(id.get(db)))
    }
}

impl Updateable for domain::Session {
    fn update<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError> {
        /*let id = match self.user_id {
            Some(id) => id,
            None => return Err(CambioError::missing_field("Session", "id"))
        };*/
        const QUERY: &'static str = "
            UPDATE user_session_info 
            SET started_at = $2, session_state = $3
            FROM user_session
            WHERE 
                user_session.session_info_id = user_session_info.id AND
                user_session.id = $1
        ";
        try!(db.execute(QUERY, &[&self.id, &self.started_at, &self.session_state]));
        self.session_token.get(db)
    }
}

impl Updateable for domain::PoliPaymentRequest {
    fn update<H: PostgresHelper>(&self, db: &mut H) -> Result<Self, CambioError> {
        let id = match self.id {
            Some(id) => id,
            None => return Err(CambioError::db_update_failed("PoliPaymentRequest"))
        };
        const QUERY: &'static str = "
            UPDATE poli_payment_request 
            SET payment_status = $2, transaction_token = $3
            WHERE id = $1
        ";
        try!(db.execute(QUERY, &[
            &self.id, &self.payment_status, &self.transaction_token
        ]));
        id.get(db)
    }
}

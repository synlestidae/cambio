use db::{CambioError, PostgresHelper};
use domain;
use postgres::GenericConnection;
use repository::Readable;
use std;

pub trait Updateable
where
    Self: std::marker::Sized,
{
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError>;
}

impl Updateable for domain::OrderSettlement {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        if let Some(id) = self.id {
            const UPDATE_SETTLEMENT: &'static str =
                "UPDATE order_settlement SET status = $2 WHERE id = $1;";
            try!(db.execute(UPDATE_SETTLEMENT, &[&self.id, &self.settlement_status]));
            id.get(db)
        } else {
            Err(CambioError::format_obj(
                "Failed to locate settlement.",
                "Settlement is missing ID field.",
            ))
        }
    }
}

impl Updateable for domain::Registration {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        let id = match self.id {
            Some(id) => id,
            None => return Err(CambioError::db_update_failed("Registration")),
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
        let update_result = db.execute(
            QUERY,
            &[
                &self.email_address,
                &self.password_hash,
                &self.confirmation_code,
                &self.identifier_code,
                &self.requested_at,
                &self.confirmed_at,
                &id,
            ],
        );
        Ok(try!(id.get(db)))
    }
}

impl Updateable for domain::Session {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
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
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        let id = match self.id {
            Some(id) => id,
            None => return Err(CambioError::db_update_failed("PoliPaymentRequest")),
        };
        const QUERY: &'static str = "
            UPDATE poli_payment_request 
            SET payment_status = $2, transaction_ref_no = $3, amount_paid_cents = $4
            WHERE id = $1
        ";
        try!(db.execute(
            QUERY,
            &[
                &self.id,
                &self.payment_status,
                &self.transaction_ref_no,
                &self.amount_paid
            ]
        ));
        id.get(db)
    }
}

impl Updateable for domain::Order {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        if let Some(id) = self.id {
            const SQL: &'static str = "UPDATE asset_order SET order_status = $2 WHERE id = $1;";
            try!(db.execute(SQL, &[&self.id, &self.status]));
            id.get(db)
        } else {
            Err(CambioError::format_obj(
                "Failed to locate order.",
                "Order is missing ID field.",
            ))
        }
    }
}

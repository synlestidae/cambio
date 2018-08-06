use db::{CambioError, PostgresHelper};
use domain;
use postgres::GenericConnection;
use repository::{Readable, Creatable};
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
                "UPDATE order_settlement SET status = $2, eth_account = $3 WHERE id = $1;";
            try!(db.execute(UPDATE_SETTLEMENT, &[&self.id, &self.status, &self.eth_account]));
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
            UPDATE session_info 
            SET started_at = $2, session_state = $3
            FROM user_session
            WHERE 
                user_session.session_info_id = session_info.id AND
                user_session.id = $1
        ";
        try!(db.execute(
            QUERY,
            &[&self.id, &self.started_at.naive_utc(), &self.session_state]
        ));
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
        const SQL: &'static str = "UPDATE asset_order SET status = $2 WHERE id = $1;";
        const SQL_ORDER_CHANGE: &'static str = 
            "INSERT INTO order_changes(order_id, field_name, old_value, new_value) 
            VALUES ($1, 'status', $2, $3)";

        if let Some(id) = self.id {
            let old_order: Self = id.get(db)?;
            db.execute(SQL, &[&id, &self.status])?;
            db.execute(SQL_ORDER_CHANGE, &[&id, &old_order.status.to_string(), &self.status.to_string()])?;
            id.get(db)
        } else {
            Err(CambioError::format_obj(
                "Failed to locate order.",
                "Order is missing ID field.",
            ))
        }
    }
}

impl Updateable for domain::Address {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        const UPDATE_ADDRESS: &'static str = "
            UPDATE address 
            SET  
               address_line_1 = $1, 
               address_line_2 = $2, 
               address_line_3 = $3, 
               address_line_4 = $4, 
               address_line_5 = $5, 
               address_line_6 = $6, 
               address_line_7 = $7, 
               country_name = $8 
            WHERE id = $9";
        db.execute(UPDATE_ADDRESS, &[
            &self.address_line_1,
            &self.address_line_2,
            &self.address_line_3,
            &self.address_line_4,
            &self.address_line_5,
            &self.address_line_6,
            &self.address_line_7,
            &self.country_name,
            &self.id],
        )?;
        Ok(self.id.unwrap().get(db)?)
    }
}

impl Updateable for domain::PersonalIdentity {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        unimplemented!()
    }
}

impl Updateable for domain::Profile {
    fn update<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        let new_address = self.address.create(db)?;
        let new_personal_identity_id = match self.personal_identity {
            Some(ref p_id) => p_id.create(db)?.id,
            None => None
        };
        const UPDATE_PROFILE: &'static str = "
            UPDATE personal_info 
            SET
                user_id = $1,
                given_names = $2,
                family_names = $3,
                date_of_birth = $4,
                address_id = $5,
                personal_identity_id = $6
            WHERE id = $7";
        db.execute(
            UPDATE_PROFILE,
            &[
                &self.user_id,
                &self.given_names,
                &self.family_names,
                &self.date_of_birth,
                &new_address.id,
                &new_personal_identity_id,
                &self.id
            ]
        )?;
        Ok(self.id.unwrap().get(db)?)
    }
}

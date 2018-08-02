use db::{CambioError, PostgresHelper};
use domain;
use payment::poli;
use postgres::rows::Rows;
use postgres::types::FromSql;
use postgres::GenericConnection;
use repository::Readable;
use services::LoggedPoliError;
use std;

pub trait Creatable
where
    Self: std::marker::Sized,
{
    type Id: Readable<Self> + FromSql;
    fn create<H: GenericConnection>(&self, db: &mut H) -> Result<Self, CambioError> {
        let update_failed = CambioError::db_update_failed("Entity");
        let result = try!(self.run_sql(db));
        if result.is_empty() {
            return Err(update_failed);
        }
        let id: Self::Id = match result.get(0).get(0) {
            Some(id) => id,
            None => return Err(update_failed),
        };
        Ok(try!(id.get(db)))
    }
    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError>;
}

impl Creatable for domain::User {
    type Id = domain::UserId;
    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str =
            "INSERT INTO users(email_address, password_hash) VALUES ($1, $2) RETURNING id;";
        //const OWNER_QUERY: &'static str = "";
        //try!(db.execute(OWNER_QUERY, &[]));
        let result = try!(db.query(QUERY, &[&self.email_address, &self.password_hash]));
        if result.len() > 0 {
            let id: domain::UserId = result.get(0).get("id");
            try!(db.execute("INSERT INTO account_owner(user_id) VALUES ($1)", &[&id]));
            Ok(result)
        } else {
            unreachable!()
        }
    }
}

impl Creatable for domain::EthAccount {
    type Id = domain::EthAccountId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str =
            "INSERT INTO ethereum_account_details(address, name, owner_id) 
             VALUES ($1, $2, $3) RETURNING id";
        Ok(try!(db.query(
            QUERY,
            &[&self.address, &self.name, &self.owner_id]
        )))
    }
}

impl Creatable for domain::Account {
    type Id = domain::AccountId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = "INSERT INTO 
            account(owner_id, asset_type, account_type, account_business_type, account_role, account_status) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id";
        let result = match db.query(
            QUERY,
            &[
                &self.owner_user_id,
                &self.asset_type,
                &self.account_type,
                &self.account_business_type,
                &self.account_role,
                &self.account_status,
            ],
        ) {
            Ok(u) => u,
            Err(err) => {
                return Err(err.into());
            }
        };
        Ok(result)
    }
}

impl Creatable for domain::Registration {
    type Id = domain::RegistrationId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = "
            INSERT INTO registration(email_address, password_hash, confirmation_code, identifier_code, requested_at, confirmed_at)
            VALUES($1, $2, $3, $4, $5, $6)
            RETURNING id
        ";
        let result = db.query(
            QUERY,
            &[
                &self.email_address,
                &self.password_hash,
                &self.confirmation_code,
                &self.identifier_code,
                &self.requested_at,
                &self.confirmed_at,
            ],
        );
        match result {
            Ok(r) => Ok(r),
            Err(err) => {
                panic!("Err {:?}", err);
                return Err(err.into());
            }
        }
    }
}

impl Creatable for domain::Session {
    type Id = domain::SessionToken;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = "SELECT activate_user_session($1)";
        let session = try!(db.query(QUERY, &[&self.email_address,]));
        Ok(session)
    }
}

impl Creatable for domain::PersonalIdentity {
    type Id = domain::Id;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const INSERT_IDENTITY: &'static str = "
            INSERT INTO personal_identity(
                user_id,
                nz_passport_number,
                nz_drivers_licence_number,
            ) 
            VALUES($1, $2, $3) RETURNING id; 
        ";
        let result = try!(db.query(
            INSERT_IDENTITY,
            &[
                &self.user_id,
                &self.nz_passport_number,
                &self.nz_drivers_licence_number
            ]
        ));
        Ok(result)
    }
}

impl Creatable for domain::Address {
    type Id = domain::Id;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const INSERT_ADDRESS: &'static str = "
            INSERT INTO address(
               address_line_1, 
               address_line_2, 
               address_line_3, 
               address_line_4, 
               address_line_5, 
               address_line_6, 
               address_line_7, 
               country_name) 
            VALUES ($1,$2, $3,$4,$5, $6, $7, $8) RETURNING id";
        let result = try!(db.query(
            INSERT_ADDRESS,
            &[
                &self.address_line_1,
                &self.address_line_2,
                &self.address_line_3,
                &self.address_line_4,
                &self.address_line_5,
                &self.address_line_6,
                &self.address_line_7,
                &self.country_name
            ]
        ));
        Ok(result)
    }
}

impl Creatable for domain::Profile {
    type Id = domain::ProfileId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        let address = try!(self.address.create(db));
        let personal_identity_id = match self.personal_identity {
            Some(ref personal_identity) => try!(personal_identity.create(db)).id,
            None => None,
        };
        const INSERT_PROFILE: &'static str = "
            INSERT INTO personal_info(
                user_id,
                given_names,
                family_names,
                date_of_birth,
                address_id,
                personal_identity_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
        ";
        let result = try!(db.query(
            INSERT_PROFILE,
            &[
                &self.user_id,
                &self.given_names,
                &self.family_names,
                &self.date_of_birth,
                &address.id,
                &personal_identity_id,
            ]
        ));
        Ok(result)
    }
}

impl Creatable for domain::Order {
    type Id = domain::OrderId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const SQL: &'static str = "INSERT INTO asset_order(
            owner_id,
            unique_id,
            amount_fiat,
            amount_crypto,
            trade_type,
            fiat_type,
            crypto_type,
            expires_at,
            status)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING id";

        const SQL_ORDER_CHANGE: &'static str = 
            "INSERT INTO order_changes(order_id, field_name, old_value, new_value) 
            VALUES ($1, 'status', NULL, $2)";

        let rows = db.query(
            SQL,
            &[
                &self.owner_id,
                &self.unique_id,
                &self.amount_fiat,
                &self.amount_crypto,
                &self.trade_type,
                &self.fiat_type,
                &self.crypto_type,
                &self.expires_at.naive_utc(),
                &self.status,
            ]
        )?;
        let order_id: domain::OrderId = rows.get(0).get("id");
        db.execute(SQL_ORDER_CHANGE, &[&order_id, &self.status.to_string()])?;
        Ok(rows)
    }
}

impl Creatable for domain::OrderSettlement {
    type Id = domain::OrderSettlementId;
    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const SQL_SETTLEMENT: &'static str = "SELECT * 
            FROM order_settlement 
            WHERE buying_crypto_id in ($1, $2) OR buying_fiat_id in ($1, $2)";
        let rows = try!(db.query(
            SQL_SETTLEMENT,
            &[&self.buying_order.id, &self.selling_order.id]
        ));
        if rows.len() > 0 {
            return Err(CambioError::not_permitted(
                "Orders can only be in one settlement at a time",
                "At least one settlement uses that order.",
            ));
        }
        const SQL: &'static str = "INSERT INTO order_settlement(
                transaction_id,
                starting_user,
                buying_crypto_id,
                buying_fiat_id
            ) VALUES (NULL, $1, $2, $3)
            RETURNING id
        ";
        let settlement = try!(db.query(
            SQL,
            &[
                &self.starting_user,
                &self.buying_order.id,
                &self.selling_order.id
            ]
        ));
        Ok(settlement)
    }
}

impl Creatable for domain::PoliPaymentRequest {
    type Id = domain::PoliPaymentRequestId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        const QUERY: &'static str = 
            "INSERT INTO poli_payment_request(user_id, amount, unique_code, started_at, payment_status, transaction_ref_no, amount_paid_cents) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) 
             RETURNING id";
        Ok(try!(db.query(
            QUERY,
            &[
                &self.user_id,
                &self.amount,
                &self.unique_code,
                &self.started_at,
                &self.payment_status,
                &self.transaction_ref_no,
                &self.amount_paid
            ]
        )))
    }
}

impl Creatable for domain::SettlementCriteria {
    type Id = domain::OrderId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        unimplemented!("Creatable<SettlementCriteria>")
    }
}

impl Creatable for domain::SettlementTransaction {
    type Id = domain::OrderSettlementId;

    fn run_sql<H: GenericConnection>(&self, db: &mut H) -> Result<Rows, CambioError> {
        unimplemented!("Creatable<SettlementTransaction>")
    }
}

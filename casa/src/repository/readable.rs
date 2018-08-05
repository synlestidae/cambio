use chrono::prelude::*;
use db::CambioError;
use db::PostgresHelper;
use db::PostgresHelperImpl;
use db::{TryFromRow, TryFromRowError};
use domain;
use payment;
use postgres;
use postgres::GenericConnection;
use repository;
use repository::RepoRead;
use repository::UserClause;

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c
pub trait Readable<Item> {
    fn get_vec<T: GenericConnection>(&self, db: &mut T) -> Result<Vec<Item>, CambioError>;
    fn get<T: GenericConnection>(&self, db: &mut T) -> Result<Item, CambioError> {
        match self.get_option(db) {
            Ok(Some(order)) => Ok(order),
            Ok(None) => Err(CambioError::not_found_search(
                "Item could not be found.",
                "No results for query.",
            )),
            Err(err) => Err(err),
        }
    }

    fn get_option<T: GenericConnection>(&self, db: &mut T) -> Result<Option<Item>, CambioError> {
        Ok(try!(self.get_vec(db)).pop())
    }
}

impl Readable<domain::User> for domain::UserId {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::User>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, users.id as user_id, account_owner.id as owner_id
            FROM users 
            JOIN account_owner ON account_owner.user_id = users.id 
            WHERE users.id = $1";

        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::User> for str {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::User>, CambioError> {
        const SELECT_BY_EMAIL: &'static str = "
            SELECT *, users.id as user_id, account_owner.id as owner_id
            FROM users 
            JOIN account_owner ON account_owner.user_id = users.id 
            WHERE users.email_address = $1";
        PostgresHelperImpl::query(db, SELECT_BY_EMAIL, &[&self])
    }
}

impl Readable<domain::Order> for domain::OrderId {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::Order>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, orders.id AS order_id
            FROM asset_order orders,
                 account_owner owners 
            WHERE orders.owner_id = owners.id AND
                  orders.id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Order> for domain::OwnerId {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::Order>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, orders.id AS order_id
            FROM asset_order orders,
                 account_owner owners 
            WHERE orders.owner_id = owners.id AND
                  owners.id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Order> for domain::All {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::Order>, CambioError> {
        const SELECT_ALL: &'static str = "
            SELECT *, orders.id AS order_id
            FROM asset_order orders,
                 account_owner owners 
            WHERE orders.owner_id = owners.id AND
                  orders.status = 'active'";
        PostgresHelperImpl::query(db, SELECT_ALL, &[])
    }
}

impl Readable<domain::Session> for domain::SessionToken {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Session>, CambioError> {
        const SELECT_BY_TOKEN: &'static str = "
            SELECT user_session.id AS session_id, session_info.*, users.email_address, users.id as user_id
            FROM user_session
            JOIN session_info ON session_info.id = user_session.session_info_id
            JOIN users ON user_session.user_id = users.id
            WHERE session_info.session_token = $1 AND 
                (now() at time zone 'utc') < (session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)))
            ORDER BY session_info.started_at";
        let result = PostgresHelperImpl::query(db, SELECT_BY_TOKEN, &[self]);
        result
    }
}

impl Readable<domain::User> for domain::OwnerId {
    fn get_vec<H: GenericConnection>(&self, db: &mut H) -> Result<Vec<domain::User>, CambioError> {
        const SELECT_BY_OWNER: &'static str = "
            SELECT *, users.id as user_id, account_owner.id as owner_id
            FROM users 
            JOIN account_owner ON account_owner.user_id = users.id 
            WHERE account_owner.id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_OWNER, &[self])
    }
}

impl Readable<domain::EthAccount> for domain::OwnerId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::EthAccount>, CambioError> {
        const SELECT_BY_OWNER: &'static str = "
            SELECT *
            FROM ethereum_account_details
            WHERE owner_id = $1";
        let result = PostgresHelperImpl::query(db, SELECT_BY_OWNER, &[self])?;
        Ok(result)
    }
}

impl Readable<domain::Account> for domain::OwnerId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Account>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, account.id as account_id, account.asset_type as account_asset_type
            FROM account 
            WHERE account.owner_id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Account> for domain::AccountId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Account>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, account.id as account_id, account.asset_type as account_asset_type
            FROM account 
            WHERE account.id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Transaction> for domain::AccountId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Transaction>, CambioError> {
        const SELECT_TRANSACTIONS: &'static str = "
            SELECT 
                journal_to.correspondence_id,
                journal_from.account_id as from_account, 
                journal_to.account_id as to_account, 
                journal_from.asset_type, 
                journal_from.debit as value, 
                journal_from.transaction_time, 
                journal_from.accounting_period as accounting_period_id,
                journal_to.balance as balance_to_account
            FROM 
                journal journal_from,
                journal journal_to
            WHERE 
                journal_to.account_id = $1 AND
                journal_from.correspondence_id = journal_to.correspondence_id AND
                journal_from.correspondence_id = $1 AND 
                journal_from.debit >= 0 AND 
                journal_to.credit >= 0
            ORDER BY journal_to.correspondence_id
        ";
        PostgresHelperImpl::query(db, SELECT_TRANSACTIONS, &[self])
    }
}

impl Readable<domain::EthAccount> for domain::EthAccountId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::EthAccount>, CambioError> {
        const SELECT_BY_ID: &'static str = "SELECT * FROM ethereum_account_details WHERE id = $1";
        PostgresHelperImpl::query(db, SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::EthAccount> for domain::ByteAddress {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::EthAccount>, CambioError> {
        const SELECT_BY_ADDRESS: &'static str = "SELECT * FROM ethereum_account_details";// WHERE encode(address, 'escape') = encode($1, 'escape')";
        let result = PostgresHelperImpl::query(db, SELECT_BY_ADDRESS, &[])?;
        Ok(result)
    }
}

impl Readable<domain::OrderSettlement> for domain::OrderSettlementId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::OrderSettlement>, CambioError> {
        const SQL: &'static str = "
            SELECT *, id as order_settlement_id
            FROM order_settlement 
            WHERE id = $1";
        PostgresHelperImpl::query(db, SQL, &[&self])
    }
}

impl Readable<domain::OrderSettlement> for domain::OrderId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::OrderSettlement>, CambioError> {
        const SQL: &'static str = "SELECT id 
            FROM order_settlement 
            WHERE buying_crypto_id = $1 OR buying_fiat_id = $1";
        PostgresHelperImpl::query(db, SQL, &[&self])
    }
}

impl Readable<domain::Registration> for domain::RegistrationId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Registration>, CambioError> {
        const SELECT_ID: &'static str = "
            SELECT * FROM registration WHERE id = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ID, &[&self])
    }
}

impl Readable<domain::Registration> for domain::IdentifierCode {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Registration>, CambioError> {
        const SELECT_ID: &'static str = "
            SELECT * FROM registration WHERE identifier_code = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ID, &[&self])
    }
}

impl Readable<domain::Profile> for domain::ProfileId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Profile>, CambioError> {
        const SELECT_PROF: &'static str = "SELECT *, users.id as user_id 
            FROM personal_info 
            JOIN users ON personal_info.user_id = users.id 
            WHERE personal_info.id = $1";
        let id: Option<domain::UserId> =
            try!(db.query(SELECT_PROF, &[&self])).get(0).get("user_id");
        match id {
            Some(id) => id.get_vec(db),
            None => Err(CambioError::missing_field("Profile", "user_id")),
        }
    }
}

impl Readable<domain::Profile> for domain::UserId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Profile>, CambioError> {
        const SELECT_ID: &'static str = "
            SELECT * 
            FROM personal_info  
            JOIN address ON personal_info.address_id = address.id
            JOIN users ON personal_info.user_id = users.id
            LEFT JOIN personal_identity ON personal_info.personal_identity_id = personal_identity.id
            LEFT JOIN contact_info ON personal_info.contact_info_id = contact_info.id
            WHERE users.id = $1";
        let mut result = vec![];
        for row in try!(db.query(SELECT_ID, &[self])).into_iter() {
            let date_of_birth: NaiveDate = match row.get("date_of_birth") {
                Some(d) => d,
                None => return Err(CambioError::missing_field("Profile", "date_of_birth")),
            };
            let personal_identity_id: Option<domain::Id> = row.get("personal_identity_id");
            let personal_identity: Option<domain::PersonalIdentity> = match personal_identity_id {
                Some(_) => Some(try!(domain::PersonalIdentity::try_from_row(&row))),
                None => None,
            };
            let address: domain::Address = try!(domain::Address::try_from_row(&row));
            let given_names: String = match row.get("given_names") {
                Some(n) => n,
                None => return Err(CambioError::missing_field("Profile", "given_names")),
            };
            let family_names: String = match row.get("family_names") {
                Some(n) => n,
                None => return Err(CambioError::missing_field("Profile", "family_names")),
            };
            let id: domain::ProfileId = match row.get("id") {
                Some(id) => id,
                None => return Err(CambioError::missing_field("Profile", "id")),
            };
            let user_id: domain::UserId = match row.get("user_id") {
                Some(uid) => uid,
                None => return Err(CambioError::missing_field("Profile", "user_id")),
            };
            result.push(domain::Profile {
                id: Some(id),
                user_id: user_id,
                given_names: given_names,
                family_names: family_names,
                date_of_birth: date_of_birth,
                //contact_details: contact_details,
                address: address,
                personal_identity: personal_identity,
            });
        }
        Ok(result)
    }
}

impl Readable<domain::Address> for domain::Id {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::Address>, CambioError> {
        const SELECT_ADDRESS: &'static str = "
            SELECT * FROM address WHERE id = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ADDRESS, &[self])
    }
}

impl Readable<domain::PersonalIdentity> for domain::Id {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::PersonalIdentity>, CambioError> {
        const SELECT_ADDRESS: &'static str = "
            SELECT * FROM personal_identity where id = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ADDRESS, &[self])
    }
}

impl Readable<domain::PoliPaymentRequest> for domain::PoliPaymentRequestId {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::PoliPaymentRequest>, CambioError> {
        const SELECT_ADDRESS: &'static str = "
            SELECT * FROM poli_payment_request where id = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ADDRESS, &[self])
    }
}

impl Readable<domain::PoliPaymentRequest> for payment::poli::TransactionToken {
    fn get_vec<H: GenericConnection>(
        &self,
        db: &mut H,
    ) -> Result<Vec<domain::PoliPaymentRequest>, CambioError> {
        const SELECT_ADDRESS: &'static str = "
            SELECT * FROM poli_payment_request where transaction_token = $1
        ";
        PostgresHelperImpl::query(db, SELECT_ADDRESS, &[self])
    }
}

impl Readable<domain::AccountId> for domain::PaymentVendor {
    fn get_vec<T: GenericConnection>(
        &self,
        db: &mut T,
    ) -> Result<Vec<domain::AccountId>, CambioError> {
        const SQL: &'static str = "SELECT intake_account FROM vendor WHERE name = $1";
        let err = Err(CambioError::not_found_search(
            "Could not process payment vendor.",
            "Payment vendor not in DB",
        ));
        let result = try!(db.query(SQL, &[&self]));
        if result.len() < 1 {
            return err;
        }
        if let Some(account_id) = result.get(0).get(0) {
            return Ok(vec![account_id]);
        }
        err
    }
}

impl Readable<domain::SettlementCriteria> for domain::OrderId {
    fn get_vec<T: GenericConnection>(
        &self,
        db: &mut T,
    ) -> Result<Vec<domain::SettlementCriteria>, CambioError> {
        const SQL: &'static str = 
            "SELECT * FROM settlement_criteria WHERE order_id = $1";
        PostgresHelperImpl::query(db, SQL, &[self])

    }
}

impl Readable<domain::SettlementTransaction> for domain::OrderSettlementId {
    fn get_vec<T: GenericConnection>(
        &self,
        db: &mut T,
    ) -> Result<Vec<domain::SettlementTransaction>, CambioError> {
        const SQL: &'static str = 
            "SELECT * FROM settlement_criteria WHERE id = $1";
        PostgresHelperImpl::query(db, SQL, &[self])
    }
}

const SELECT_BY_OWNER: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE account_owner.id = $1";

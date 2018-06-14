use chrono::prelude::*;
use db::CambioError;
use db::PostgresHelper;
use db::{TryFromRow, TryFromRowError};
use query::Selectable;
use domain;
use postgres;
use repositories;
use repository;
use repository::RepoRead;
use repository::UserClause;

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c
pub trait Readable<Item> {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<Item>, CambioError>;
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<Item, CambioError> {
        match self.get_option(db) {
            Ok(Some(order)) => Ok(order),
            Ok(None) => Err(CambioError::not_found_search(
                "Item could not be found.",
                "No results for query.",
            )),
            Err(err) => Err(err),
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: &mut H) -> Result<Option<Item>, CambioError> {
        Ok(try!(self.get_vec(db)).pop())
    }
}

impl Readable<domain::User> for domain::UserId {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::User>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT *, users.id as user_id, account_owner.id as owner_id
            FROM users 
            JOIN account_owner ON account_owner.user_id = users.id 
            WHERE users.id = $1";

        db.query(SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Order> for domain::OrderId {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::Order>, CambioError> {
        const SELECT_BY_ID: &'static str = "
            SELECT 
                *, 
                orders.id AS order_id, 
                sell_asset_type.asset_code AS sell_asset_code,  
                sell_asset_type.denom AS sell_asset_denom,  
                buy_asset_type.asset_code AS buy_asset_code,  
                buy_asset_type.denom AS buy_asset_denom
            FROM asset_order orders,
                 account_owner owners, 
                 asset_type buy_asset_type, 
                 asset_type sell_asset_type
            WHERE orders.owner_id = owners.id AND
                  buy_asset_type.id = orders.buy_asset_type_id AND
                  sell_asset_type.id = orders.sell_asset_type_id AND
                  orders.id = $1";
        db.query(SELECT_BY_ID, &[self])
    }
}

impl Readable<domain::Session> for domain::SessionToken {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::Session>, CambioError> {
        const SELECT_BY_TOKEN: &'static str = "
            SELECT user_session.id AS session_id, session_info.*, users.email_address, users.id as user_id
            FROM user_session
            JOIN session_info ON session_info.id = user_session.session_info_id
            JOIN users ON user_session.user_id = users.id
            WHERE session_info.session_token = $1 AND 
                (now() at time zone 'utc') < (session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)))";
        db.query(SELECT_BY_TOKEN, &[self])
    }
}

impl Readable<domain::User> for domain::OwnerId {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::User>, CambioError> {
        const SELECT_BY_OWNER: &'static str = "
            SELECT *, users.id as user_id, account_owner.id as owner_id
            FROM users 
            JOIN account_owner ON account_owner.user_id = users.id 
            WHERE account_owner.id = $1";
        db.query(SELECT_BY_OWNER, &[self])
    }
}

impl Readable<domain::EthAccount> for domain::OwnerId {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::EthAccount>, CambioError> {
        const SELECT_BY_OWNER: &'static str = "
            SELECT *
            FROM ethereum_account_details
            WHERE owner_id = $1";
        db.query(SELECT_BY_OWNER, &[self])
    }
}

impl<E> Readable<E> for Selectable<E> where E: TryFromRow {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<E>, CambioError> {
        let sql = self.get_specifier().get_sql_query();
        db.query(&sql, &[])
    }
}

#[derive(TryFromRow)]
struct SettlementRow {
    pub id: Option<domain::OrderSettlementId>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: domain::UserId,
    pub settlement_status: domain::SettlementStatus,
    pub buying_crypto_id: domain::OrderId,
    pub selling_crypto_id: domain::OrderId,
}

impl Readable<domain::OrderSettlement> for domain::OrderSettlementId {
    fn get_vec<H: PostgresHelper>(&self, db: &mut H) -> Result<Vec<domain::OrderSettlement>, CambioError> {
        unimplemented!()
    }

    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<domain::OrderSettlement, CambioError> {
        match self.get_option(db) {
            Ok(Some(order_settlement)) => Ok(order_settlement),
            Ok(None) => Err(CambioError::not_found_search(
                "Settlement with that ID could not be found.",
                "No matches for a settlement with that ID.",
            )),
            Err(err) => Err(err),
        }
    }

    fn get_option<H: PostgresHelper>(
        &self,
        db: &mut H,
    ) -> Result<Option<domain::OrderSettlement>, CambioError> {
        const SELECT: &'static str = "SELECT * FROM order_settlement WHERE id = $1";
        let result: Option<SettlementRow> = try!(db.query(SELECT, &[&self.0])).pop();
        match result {
            Some(row) => {
                let buying_crypto: domain::Order = try!(row.buying_crypto_id.get(db));
                let selling_crypto: domain::Order = try!(row.selling_crypto_id.get(db));
                Ok(Some(domain::OrderSettlement {
                    id: Some(self.clone()),
                    started_at: row.started_at,
                    settled_at: row.settled_at,
                    starting_user: row.starting_user,
                    settlement_status: row.settlement_status,
                    buying_order: buying_crypto,
                    selling_order: selling_crypto,
                }))
            }
            None => Ok(None),
        }
    }
}

const SELECT_BY_OWNER: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE account_owner.id = $1";

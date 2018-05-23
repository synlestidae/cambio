use db::CambioError;
use db::PostgresHelper;
use repository::UserClause;
use repository::RepoRead;
use repositories;
use repository;
use domain;
use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use postgres;
//use chrono::prelude::{DateTime, Utc};

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c

pub trait Retrievable<Item> {
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<Item, CambioError>; 
    fn get_option<H: PostgresHelper>(&self, db: &mut H)  -> Result<Option<Item>, CambioError>;
}

impl Retrievable<domain::Order> for domain::OrderId {
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<domain::Order, CambioError> {
        match self.get_option(db) {
            Ok(Some(order)) => Ok(order),
            Ok(None) => Err(CambioError::not_found_search("Order could not be found.", 
                "No results for that Order ID.")),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: &mut H)  -> Result<Option<domain::Order>, CambioError> {
        let clause = repository::UserClause::Id(self.clone().into());
        let mut order_repo = repositories::OrderRepository::new(db.clone());
        order_repo.read(&clause).map(|mut s| s.pop())
    }
}

impl Retrievable<domain::Session> for domain::SessionToken {
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<domain::Session, CambioError> {
        match self.get_option(db) {
            Ok(Some(session_token)) => Ok(session_token),
            Ok(None) => Err(CambioError::unauthorised()),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: &mut H)  -> Result<Option<domain::Session>, CambioError> {
        let clause = repository::UserClause::SessionToken(self.0.to_owned());
        let mut session_repo = repositories::SessionRepository::new(db.clone());
        session_repo.read(&clause).map(|mut s| s.pop())
    }
}


impl Retrievable<domain::User> for domain::OwnerId {
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<domain::User, CambioError> {
        match self.get_option(db) {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CambioError::not_found_search("User could not be located.", 
                "User with owner ID not found")),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: &mut H)  -> Result<Option<domain::User>, CambioError> {
        let mut matches = try!(db.query(SELECT_BY_OWNER, &[&self.0]));
        Ok(matches.pop())
    }
}

#[derive(TryFromRow)]
struct SettlementRow {
    pub id: Option<domain::OrderSettlementId>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: domain::Id,
    pub settlement_status: domain::SettlementStatus,
    pub buying_crypto_id: domain::OrderId,
    pub selling_crypto_id: domain::OrderId,
}

impl Retrievable<domain::OrderSettlement> for domain::OrderSettlementId {
    fn get<H: PostgresHelper>(&self, db: &mut H) -> Result<domain::OrderSettlement, CambioError> {
        match self.get_option(db) {
            Ok(Some(order_settlement)) => Ok(order_settlement),
            Ok(None) => Err(CambioError::not_found_search(
                "Settlement with that ID could not be found.", 
                "No matches for a settlement with that ID.")),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, db: &mut H)  -> Result<Option<domain::OrderSettlement>, CambioError> {
        const SELECT: &'static str = "SELECT * FROM order_settlement WHERE id = $1";
        let result: Option<SettlementRow>  = try!(db.query(SELECT, &[&self.0])).pop();
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
                    selling_order: selling_crypto
                }))
            },
            None => Ok(None)
        }
    }
}

const SELECT_BY_OWNER: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE account_owner.id = $1";

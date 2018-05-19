use chrono::prelude::*;
use db::{CambioError, TryFromRow, TryFromRowError};
use db;
use domain::{Id, OrderSettlement};
use domain;
use postgres::types::ToSql;
use postgres;
use repositories::OrderRepository;
use repository;
use repository::{RepoCreate, RepoRead, RepoUpdate};

#[derive(Clone)]
pub struct SettlementRepository<T: db::PostgresHelper> {
    order_repository: OrderRepository<T>,
    db_helper: T,
}

impl<T: db::PostgresHelper> SettlementRepository<T> {
    pub fn new(db: T) -> Self {
        Self {
            order_repository: OrderRepository::new(db.clone()),
            db_helper: db,
        }
    }

    fn _add_orders(&mut self, row: SettlementRow) -> Result<OrderSettlement, CambioError> {
        let buying_order = try!(
            self.order_repository
                .read(&repository::UserClause::Id(row.buying_crypto_id))
        ).pop();
        let selling_order = try!(
            self.order_repository
                .read(&repository::UserClause::Id(row.buying_fiat_id))
        ).pop();
        match (buying_order, selling_order) {
            (Some(b), Some(s)) => Ok(OrderSettlement {
                id: row.id,
                started_at: row.started_at,
                settled_at: row.settled_at,
                starting_user: row.starting_user,
                settlement_status: row.settlement_status,
                buying_order: b,
                selling_order: s,
            }),
            _ => Err(db::CambioError::not_found_search(
                "Cannot find buying and/or selling order for settlement",
                "Either buying or selling order does not exist in DB",
            )),
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for SettlementRepository<T> {
    type Item = domain::OrderSettlement;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        let sql_items: (&str, Vec<&ToSql>) = match clause {
            &repository::UserClause::All(true) => (SELECT_ALL, vec![]),
            &repository::UserClause::All(false) => (SELECT_ACTIVE, vec![]),
            &repository::UserClause::Id(ref id) => (SELECT_ID, vec![id]),
            &repository::UserClause::EmailAddress(ref e) => (SELECT_EMAIL, vec![e]),
            _ => {
                return Err(db::CambioError::format_obj(
                    "Don't have the right query to find settlement",
                    "Clause not support",
                ))
            }
        };
        let (sql, params) = sql_items;
        let rows: Vec<SettlementRow> = try!(self.db_helper.query(sql, &params));
        let mut settlements = vec![];
        for row in rows.into_iter() {
            let settlement = try!(self._add_orders(row));
            settlements.push(settlement);
        }
        Ok(settlements)
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for SettlementRepository<T> {
    type Item = domain::OrderSettlement;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        try!(self.db_helper.execute(
            BEGIN_SETTLEMENT,
            &[
                &item.buying_order.id,
                &item.selling_order.id,
                &item.starting_user
            ]
        ));

        let mut rows = try!(self.db_helper.query(
            SELECT_ORDERS,
            &[&item.buying_order.id, &item.selling_order.id]
        ));

        let s = match rows.pop() {
            Some(s) => s,
            None => return Err(CambioError::db_update_failed("OrderSettlement")),
        };

        self._add_orders(s)
    }
}

impl<T: db::PostgresHelper> repository::RepoUpdate for SettlementRepository<T> {
    type Item = domain::OrderSettlement;

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let id = match item.id {
            Some(id) => id,
            _ => {
                return Err(db::CambioError::format_obj(
                    "Cannot update order without ID",
                    "Order id was None",
                ))
            }
        };
        self.db_helper
            .execute(UPDATE_SETTLEMENT, &[&id, &item.settlement_status]);
        let updated_settlement = try!(self.read(&repository::UserClause::Id(id))).pop();
        match updated_settlement {
            Some(s) => Ok(s),
            _ => Err(db::CambioError::db_update_failed("OrderSettlement")),
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoDelete for SettlementRepository<T> {
    type Item = domain::OrderSettlement;

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let id = match item.id {
            Some(ref id) => id,
            None => {
                return Err(CambioError::format_obj(
                    "Cannot remove a settlement without ID",
                    "Item ID was None",
                ))
            }
        };
        Err(CambioError::shouldnt_happen(
            "Settlements can't be deleted or cancelled.",
            "Cannot delete a settlement",
        ))
    }
}

struct SettlementRow {
    pub id: Option<Id>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: Id,
    pub settlement_status: domain::SettlementStatus,
    pub buying_crypto_id: Id,
    pub buying_fiat_id: Id,
}

// TODO this will panic if row format is slightly off
impl TryFromRow for SettlementRow {
    fn try_from_row<'a>(row: &postgres::rows::Row<'a>) -> Result<Self, db::TryFromRowError> {
        let id: Option<Id> = row.get("id");
        let started_at: NaiveDateTime = row.get("started_at");
        let settled_at: Option<NaiveDateTime> = row.get("settled_at");
        let starting_user: Id = row.get("starting_user");
        let settlement_status: domain::SettlementStatus = row.get("status");
        let buying_crypto_id: Id = row.get("buying_crypto_id");
        let buying_fiat_id: Id = row.get("buying_fiat_id");

        Ok(SettlementRow {
            id: id,
            started_at: DateTime::from_utc(started_at, Utc),
            settled_at: settled_at.map(|s| DateTime::from_utc(s, Utc)),
            starting_user: starting_user,
            settlement_status: settlement_status,
            buying_crypto_id: buying_crypto_id,
            buying_fiat_id: buying_fiat_id,
        })
    }
}

const BEGIN_SETTLEMENT: &'static str = "
    SELECT begin_settlement($1, $2, $3);
";

const UPDATE_SETTLEMENT: &'static str = "
    UPDATE asset_order SET order_status = $2;";

const SELECT_ID: &'static str = "
    SELECT * FROM order_settlement where id = $1
";

const SELECT_ORDERS: &'static str = "
    SELECT * FROM order_settlement where buying_crypto_id = $1 AND buying_fiat_id = $2
";

const SELECT_ALL: &'static str = "
    SELECT * FROM order_settlement
";

const SELECT_EMAIL: &'static str = "
    SELECT * FROM order_settlement
";

const SELECT_ACTIVE: &'static str = "
    SELECT * FROM order_settlement WHERE status = 'settling'
";

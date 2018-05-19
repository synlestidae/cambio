use repository;
use db;
use domain;
use postgres::types::ToSql;
use repository::*;

#[derive(Clone)]
pub struct OrderRepository<T: db::PostgresHelper> {
    db_helper: T,
}

impl<T: db::PostgresHelper> OrderRepository<T> {
    pub fn new(db: T) -> Self {
        OrderRepository { db_helper: db }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for OrderRepository<T> {
    type Item = domain::Order;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        match clause {
            &repository::UserClause::All(include_all) => if include_all {
                self.db_helper.query(SELECT_ALL, &[])
            } else {
                self.db_helper.query(SELECT_ALL_ACTIVE, &[])
            },
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::UserClause::EmailAddress(ref email_address) => {
                self.db_helper.query(SELECT_BY_ID, &[email_address])
            }
            &repository::UserClause::UniqueId(ref unique_id) => {
                self.db_helper.query(SELECT_BY_UID, &[unique_id])
            }
            _ => Err(db::CambioError::shouldnt_happen(
                "Cannot load orders with query",
                "Unsupported query",
            )),
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for OrderRepository<T> {
    type Item = domain::Order;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        if item.is_expired() {
            return Err(db::CambioError::not_permitted(
                "Cannot create an order that has expired.",
                "Order has expired",
            ));
        }
        let params: &[&ToSql] = &[
            &item.buy_asset_type,
            &item.buy_asset_denom,
            &item.sell_asset_type,
            &item.sell_asset_denom,
            &item.unique_id,
            &item.owner_id,
            &item.sell_asset_units,
            &item.buy_asset_units,
            &item.expires_at.naive_utc(),
        ];
        let rows = try!(self.db_helper.execute(PLACE_ORDER, params));
        if rows == 0 {
            Err(db::CambioError::db_update_failed("Order"))
        } else {
            let clause = repository::UserClause::UniqueId(item.unique_id.to_owned());
            let mut orders = try!(self.read(&clause));
            match orders.pop() {
                Some(order) => Ok(order),
                None => Err(db::CambioError::db_update_failed("Order")),
            }
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoUpdate for OrderRepository<T> {
    type Item = domain::Order;
    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let id = match item.id {
            Some(id) => id,
            _ => {
                return Err(db::CambioError::format_obj(
                    "Cannot find Order without ID",
                    "Order was
                    None",
                ));
            }
        };
        let result = self.db_helper.execute(
            UPDATE_BY_ID,
            &[&id, &item.status, &item.expires_at.naive_utc()],
        );
        let rows = try!(result);
        if rows == 0 {
            Err(db::CambioError::shouldnt_happen(
                "Cannot load orders with query",
                "Unsupported query",
            ))
        } else {
            let clause = repository::UserClause::UniqueId(item.unique_id.to_owned());
            match try!(self.read(&clause)).pop() {
                Some(order) => Ok(order),
                _ => Err(db::CambioError::db_update_failed("Order")),
            }
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoDelete for OrderRepository<T> {
    type Item = domain::Order;
    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let mut order_match = if let Some(id) = item.id {
            try!(self.read(&repository::UserClause::Id(id))).pop()
        } else {
            return Err(db::CambioError::format_obj(
                "Cannot cancel order with no ID",
                "delete(): item.id was None",
            ));
        };
        match order_match {
            Some(mut order) => {
                if order.status == domain::OrderStatus::Active {
                    order.status = domain::OrderStatus::Deleted;
                    self.update(&order)
                } else {
                    Err(db::CambioError::format_obj(
                        "Can only mark an active order as deleted",
                        "delete(): item.id was None",
                    ))
                }
            }
            None => Err(db::CambioError::not_found_search(
                "Order with that ID not found",
                "Order with ID does not exist",
            )),
        }
    }
}

const SELECT_ALL: &'static str = "
    SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         asset_type buy_asset_type, 
         asset_type sell_asset_type,
         account_owner owner 
    WHERE 
        orders.buy_asset_type_id = buy_asset_type.id AND
        orders.sell_asset_type_id = sell_asset_type.id AND
        orders.owner_id = owner.id";

const SELECT_ALL_ACTIVE: &'static str = "
    SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         asset_type buy_asset_type, 
         asset_type sell_asset_type,
         account_owner owner 
    WHERE 
        (orders.status = 'active' OR orders.status = 'settling') AND
        orders.buy_asset_type_id = buy_asset_type.id AND
        orders.sell_asset_type_id = sell_asset_type.id AND
        orders.owner_id = owner.id AND
        now() at time zone 'utc' < orders.expires_at";

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

const UPDATE_BY_ID: &'static str = "
    UPDATE asset_order
    SET status = $2, expires_at = $3
    WHERE id = $1
";

const UPDATE_BY_UID: &'static str = "
    UPDATE asset_order
    SET _status = $2, expires_at = $3
    WHERE unique_id = $1
";

const SELECT_BY_UID: &'static str = "SELECT 
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
          orders.unique_id = $1";

const PLACE_ORDER: &'static str = "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_BY_EMAIL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         account_owner owners, 
         users users, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type
    WHERE orders.owner_id = owners.id AND
          buy_asset_type.id = orders.buy_asset_type_id AND
          sell_asset_type.id = orders.sell_asset_type_id AND 
          users.id = owners.user_id AND
          users.email_address = $1";

const SELECT_ORDERS_IN_SETTLEMENT_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        settlements.id as settlement_id, 
        sell_asset_type.asset_code AS sell_asset_code,  
        sell_asset_type.denom AS sell_asset_denom,  
        buy_asset_type.asset_code AS buy_asset_code,  
        buy_asset_type.denom AS buy_asset_denom
    FROM asset_order orders,
         asset_order cp_order,
         account_owner owners, 
         asset_type buy_asset_type, 
         asset_type sell_asset_type,
         order_settlement settlements
    WHERE 
        order_settlement.buying_crypto_id = orders.id OR 
        order_settlement.buying_fiat_id = orders.id";

use chrono::prelude::*;
use db::{CambioError, PostgresHelper};
use domain;
use domain::{OrderId, Order, OrderStatus, AssetType};
use repositories;
use repository;
use repository::*;
use web3::types::U256;
use postgres::GenericConnection;

#[derive(Clone)]
pub struct OrderService { }

const ORDER_TIME_MINUTES: i64 = 10;

impl OrderService {
    pub fn new() -> Self {
        Self { }
    }

    pub fn place_order<C: GenericConnection>(
        &self,
        db: &mut C,
        email: &str,
        unique_id: &str,
        sell_units: u64,
        sell_currency: AssetType,
        buy_units: u64,
        buy_currency: AssetType,
        wei_cost: Option<U256>
    ) -> Result<Order, CambioError> {
        let user = try!(Readable::read(email, db));
        let user_owner_id = match user {
            None => {
                return Err(CambioError::not_found_search(
                    "Cannot find user for order",
                    "UserRepository.read() returned None",
                ))
            }
            Some(user) => user.owner_id.unwrap(),
        };
        let order = Order {
            id: None,
            owner_id: user_owner_id,
            unique_id: unique_id.to_owned(),
            sell_asset_units: sell_units as i64,
            buy_asset_units: buy_units as i64,
            sell_asset_type: sell_currency,
            buy_asset_type: buy_currency,
            expires_at: self.get_order_expiry(),
            status: OrderStatus::Active,
            max_wei: wei_cost
        };
        order.create(db)
    }

    fn get_order_expiry(&self) -> DateTime<Utc> {
        use time::Duration;
        let now = Utc::now();
        now + Duration::minutes(ORDER_TIME_MINUTES)
    }

    pub fn cancel_order<C: GenericConnection>(&self, db: &mut C, order_id: OrderId) 
        -> Result<Order, CambioError> {
        let mut order: Order = try!(order_id.read(db)); //self.order_repo.read(&order_clause);
        if order.status == domain::OrderStatus::Active {
            order.status = domain::OrderStatus::Deleted;
            Ok(order.update(db))
        } else {
            Err(CambioError::not_permitted(
                "Can only cancel an active order",
                "Can only change order status if status = 'active'",
            ))
        }
    }

    pub fn get_orders<C: GenericConnection>(
        &self,
        db: &mut C,
        user: Option<&str>,
        only_active: bool,
    ) -> Result<Vec<Order>, CambioError> {
        let clause: repository::UserClause;
        clause = match user {
            None => repository::UserClause::All(only_active),
            Some(email_address) => repository::UserClause::EmailAddress(email_address.to_owned()),
        };
        let orders = try!(self.order_repo.read(&clause))
            .into_iter()
            .filter(|order| !only_active || order.status == domain::OrderStatus::Active)
            .collect();

        Ok(orders)
    }
}

const USER_FROM_ORDER: &'static str = "SELECT *, users.id as user_id from users 
    JOIN account_owner ON account_owner.user_id = users.id 
    JOIN orders ON orders.owner_id = account_owner.id
    WHERE orders.id = $1";

const INSERT_NEW_ORDER_SQL: &'static str =
    "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_ORDER_UNIQUE_ID_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id
    FROM asset_order orders,
         account_owner owners
    WHERE orders.owner_id = owners.id AND
          owners.id = $1 AND
          orders.unique_id = $2";

const SELECT_ORDER_BY_ID_SQL: &'static str = "
    SELECT *, orders.id AS order_id
    FROM asset_order orders,
         account_owner owners
    WHERE orders.owner_id = owners.id AND
          orders.id = $1";

const UPDATE_ORDERS_EXPIRED_SQL: &'static str = "UPDATE asset_orders SET status = 'expires' 
    WHERE status = 'active' AND expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id
    FROM asset_order orders,
         account_owner owners, 
    WHERE orders.status = 'active' AND orders.expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id
    FROM asset_order orders,
         account_owner owners, 
         users users, 
    WHERE orders.owner_id = owners.id AND
          users.id = owners.user_id AND
          users.email_address = $1";

const SELECT_ORDERS_IN_SETTLEMENT_SQL: &'static str = "SELECT 
        *, 
        orders.id AS order_id, 
        settlements.id as settlement_id
    FROM asset_order orders,
         asset_order cp_order,
         account_owner owners, 
         order_settlement settlements
    WHERE 
        order_settlement.buying_crypto_id = orders.id OR 
        order_settlement.buying_fiat_id = orders.id
";

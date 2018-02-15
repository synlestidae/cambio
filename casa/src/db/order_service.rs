use chrono::prelude::*;
use db::{CambioError, PostgresHelper};
use domain::{Order, Id, Currency, OrderStatus};
use domain;
use repositories;
use repository::Repository;
use repository;

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    user_repo: repositories::UserRepository<T>,
    order_repo: repositories::OrderRepository<T>,
    account_repo: repositories::AccountRepository<T>
}

const ORDER_TIME_MINUTES: i64 = 10;

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { 
            user_repo: repositories::UserRepository::new(db_helper.clone()),
            order_repo: repositories::OrderRepository::new(db_helper.clone()),
            account_repo: repositories::AccountRepository::new(db_helper.clone())
        }
    }

    pub fn place_order(&mut self, 
        email: &str, 
        unique_id: &str,
        sell_units: u64, 
        sell_currency: Currency,
        buy_units: u64, 
        buy_currency: Currency) -> Result<Order, CambioError> {
        let user_clause = repository::UserClause::EmailAddress(email.to_owned());
        let user = try!(self.user_repo.read(&user_clause)).pop();
        let user_owner_id = match user {
            None => return Err(CambioError::not_found_search("Cannot find user for order", 
                "UserRepository.read() returned None")),
            Some(user) => user.owner_id.unwrap()
        };
        let order = Order {
            id: None,
            owner_id: user_owner_id,
            unique_id: unique_id.to_owned(),
            sell_asset_units: sell_units as i64,
            buy_asset_units: buy_units as i64,
            sell_asset_type: sell_currency.asset_type,
            sell_asset_denom: sell_currency.denom,
            buy_asset_type: buy_currency.asset_type,
            buy_asset_denom: buy_currency.denom,
            expires_at: self.get_order_expiry(),
            status: OrderStatus::Active
        };
        self.order_repo.create(&order)
    }

    fn get_order_expiry(&self) -> DateTime<Utc> {
        use time::Duration;
        let now = Utc::now();
        now + Duration::minutes(ORDER_TIME_MINUTES)
    }

    pub fn cancel_order(&mut self, order_id: Id) -> Result<Option<Order>, CambioError> {
        let order_clause = repository::UserClause::Id(order_id);
        let order_result = self.order_repo.read(&order_clause);
        let mut order_match = try!(order_result).pop();
        match order_match {
            None => Ok(None),
            Some(mut modifying_order) => {
                if modifying_order.status == domain::OrderStatus::Active {
                    try!(self.order_repo.delete(&modifying_order));
                    Ok(try!(self.order_repo.read(&order_clause)).pop())
                } else {
                    Err(CambioError::not_permitted("Can only cancel an active order", 
                        "Can only change order status if status = 'active'"))
                }
            }
        }
    }

    pub fn get_orders(&mut self, user: Option<&str>, only_active: bool) 
        -> Result<Vec<Order>, CambioError> {
        let clause: repository::UserClause;
        clause = match user {
            None => repository::UserClause::All(only_active),
            Some(email_address) => repository::UserClause::EmailAddress(email_address.to_owned())
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

const INSERT_NEW_ORDER_SQL: &'static str = "SELECT place_order($1, $2, $3, $4, $5, $6, $7, $8, $9);";

const SELECT_ORDER_UNIQUE_ID_SQL: &'static str = "SELECT 
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
          owners.id = $1 AND
          orders.unique_id = $2";

const SELECT_ORDER_BY_ID_SQL: &'static str = "
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

const UPDATE_ORDERS_EXPIRED_SQL: &'static str = "UPDATE asset_orders SET status = 'expires' 
    WHERE status = 'active' AND expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_SQL: &'static str = "SELECT 
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
    WHERE orders.status = 'active' AND orders.expires_at >= (now() at time zone 'utc');";

const SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL: &'static str = "SELECT 
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
        order_settlement.buying_fiat_id = orders.id
";


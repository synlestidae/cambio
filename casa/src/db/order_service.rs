use chrono::prelude::*;
use db::{PostgresHelper, AccountService, CambioError, UserService};
use domain::{Order, OrderSettlement, OrderSettlementBuilder, Id, User, AccountBusinessType, Account, SessionState};
use repositories::UserRepository;
use repositories;
use repository::Repository;
use repository;
use domain;

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    account_service: AccountService<T>,
    user_repo: repositories::UserRepository<T>,
    order_repo: repositories::OrderRepository<T>,
    account_repo: repositories::AccountRepository<T>
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { 
            account_service: AccountService::new(db_helper.clone()),
            user_repo: UserRepository::new(db_helper.clone()),
            order_repo: repositories::OrderRepository::new(db_helper.clone()),
            account_repo: repositories::AccountRepository::new(db_helper.clone())
        }
    }

    pub fn place_order(
        &mut self,
        owner_id: Id,
        order: &Order,
    ) -> Result<Order, CambioError> {
        //order.owner_id = owner_id;
        self.order_repo.create(&order)
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

    pub fn get_all_active_orders(&mut self) -> Result<Vec<Order>, CambioError> {
        unimplemented!()
       // let active_orders = try!(self.db_helper.query(SELECT_ALL_ACTIVE_ORDERS_SQL, &[]));
       // Ok(active_orders)
    }

    pub fn get_all_active_orders_by_user(
        &mut self,
        email_address: &str,
    ) -> Result<Vec<Order>, CambioError> {
        self.order_repo.read(&repository::UserClause::EmailAddress(email_address.to_owned()))
    }

    pub fn get_order_settlement_status(
        &mut self,
        order_id: Id,
    ) -> Result<Option<OrderSettlement>, CambioError> {
        unimplemented!();
    }

    // ALL METHODS MUST BE IMMUNE TO REPLAY ATTACKS
    pub fn begin_order_settlement(&mut self, buying_crypto_order: &Order, selling_order: &Order) 
        -> Result<OrderSettlement, CambioError> {
            unimplemented!()
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


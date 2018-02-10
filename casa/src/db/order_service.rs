use chrono::prelude::*;
use db::{PostgresHelper, AccountRepository, CambioError, UserService};
use domain::{Order, OrderSettlement, OrderSettlementBuilder, Id, User, AccountBusinessType, Account};
use repositories;
use repository::Repository;

#[derive(Clone)]
pub struct OrderService<T: PostgresHelper> {
    account_repository: AccountRepository<T>,
    user_repo: repositories::UserRepository<T>,
    db_helper: T
}

impl<T: PostgresHelper> OrderService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { 
            account_repository: AccountRepository::new(db_helper.clone()),
            user_repo: UserRepository::new(db_helper.clone()),
            db_helper: db_helper 
        }
    }

    pub fn place_order(
        &mut self,
        owner_id: Id,
        order: &Order,
    ) -> Result<Order, CambioError> {
        let sell_asset_units = order.sell_asset_units as i64;
        let buy_asset_units = order.buy_asset_units as i64;

        let execute_result = self.db_helper.execute(
            INSERT_NEW_ORDER_SQL,
            &[
                &order.buy_asset_type,
                &order.buy_asset_denom,
                &order.sell_asset_type,
                &order.sell_asset_denom,
                &order.unique_id,
                &owner_id,
                &sell_asset_units,
                &buy_asset_units,
                &order.expires_at.naive_utc(),
            ],
        );

        let rows = try!(execute_result);
        let new_order = try!(self.get_order_by_unique_id(owner_id, &order.unique_id));
        let mut retrieve_error = CambioError::shouldnt_happen(
            "Your order could not be retrieved. Check the Orders page and try again", 
            "get_order_by_unique_id returned None");
        new_order.ok_or(retrieve_error)
    }

    pub fn cancel_order(&mut self, order_id: Id) -> Result<Option<Order>, CambioError> {
        try!(self.db_helper.execute(CANCEL_ORDER_SQL, &[&order_id]));
        self.get_order_by_id(order_id)
    }

    pub fn get_all_active_orders(&mut self) -> Result<Vec<Order>, CambioError> {
        let active_orders = try!(self.db_helper.query(SELECT_ALL_ACTIVE_ORDERS_SQL, &[]));
        Ok(active_orders)
    }

    pub fn get_all_active_orders_by_user(
        &mut self,
        email_address: &str,
    ) -> Result<Vec<Order>, CambioError> {
        let orders = try!(self.db_helper.query(
            SELECT_ALL_ACTIVE_ORDERS_BY_USER_SQL,
            &[&email_address],
        ));
        Ok(orders)
    }

    pub fn get_order_settlement_status(
        &mut self,
        order_id: Id,
    ) -> Result<Option<OrderSettlement>, CambioError> {
        let settlement_result = self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id]);
        let settlement: OrderSettlementBuilder;

        if let Some(s) = try!(settlement_result).pop() {
            settlement = s;
        } else {
            return Ok(None);
        }

        let settlement_id = settlement.id.unwrap();

        let mut orders_in_settlement_result: Vec<OrderSettlementBuilder> =
            try!(self.db_helper.query(
                SELECT_ORDERS_IN_SETTLEMENT_SQL,
                &[&settlement_id],
            ));

        let order_settlement_builder: OrderSettlementBuilder;

        if let Some(s) = orders_in_settlement_result.pop() {
            order_settlement_builder = s;
        } else {
            return Ok(None);
        }

        let mut order_result: Vec<Order> = try!(self.db_helper.query(
            SELECT_ORDERS_IN_SETTLEMENT_SQL,
            &[&settlement_id],
        ));

        let buying_order: Order;
        let selling_order: Order;
        if order_result.len() != 2 {
            let sys_message = format!("Expected 2 orders, got {}", order_result.len());
            return Err(CambioError::shouldnt_happen("Failed to match the orders in settlement. ", &sys_message));
        }
        buying_order = order_result.pop().unwrap();
        selling_order = order_result.pop().unwrap();

        let settlement = order_settlement_builder.build(buying_order, selling_order);

        Ok(Some(settlement))
    }

    pub fn get_order_by_id(&mut self, order_id: Id) -> Result<Option<Order>, CambioError> {
        let mut orders = try!(self.db_helper.query(SELECT_ORDER_BY_ID_SQL, &[&order_id])); 
        Ok(orders.pop())
    }

    pub fn get_order_by_unique_id(
        &mut self,
        owner_id: Id,
        unique_id: &str,
    ) -> Result<Option<Order>, CambioError> {
        let order_result = self.db_helper.query(
            SELECT_ORDER_UNIQUE_ID_SQL,
            &[&owner_id, &unique_id],
        );

        let mut order_list = try!(order_result);
        Ok(order_list.pop())
    }

    // ALL METHODS MUST BE IMMUNE TO REPLAY ATTACKS
    pub fn begin_order_settlement(&mut self, buying_crypto_order: &Order, selling_order: &Order) 
        -> Result<OrderSettlement, CambioError> {
            let buy_id = buying_crypto_order.id.unwrap();
            let sell_id = selling_order.id.unwrap();
            // steps to settle 
            // -1 check that orders aren't settled already
            let buying_settlement = try!(self.get_order_settlement_status(buy_id));
            let selling_settlement = try!(self.get_order_settlement_status(sell_id));
            if buying_settlement.is_none() || selling_settlement.is_none() {
                    return Err(CambioError::unfair_operation("At least one order is already in settlement.",
                        "At least one order has an existing settlement status."));
            }
            // 0 check that past transactions haven't already been made for this order
            // TODO! 
            // retrieve two orders from DB 

            // check orders both match - they must be what user is looking for
            if !selling_order.is_fair(&buying_crypto_order) {
                return Err(CambioError::unfair_operation("Cannot settle two orders who aren't mutual.",
                    "Order is_fair returned false"));
                unimplemented!() // TODO return an error 
            }

            // retrieve the monetary account for the fiat-currency receiver
            let buying_user = self.get_order_owner(buy_id).unwrap();
            let selling_user = self.get_order_owner(sell_id).unwrap();

            // retrieve the ethereum account for the crypto-currency seller 
            let account = try!(self._get_order_account(sell_id, selling_order));

            // check that both accounts have sufficient funds
            let account_id = account.id.unwrap();
            let statement = try!(self.account_repository.get_latest_statement(account_id));
            if statement.closing_balance < buying_crypto_order.sell_asset_units {
                let mut error = CambioError::unfair_operation("Insufficient funds to buy crypto",
                    "User closing balance is less than order value");
                return Err(error);
            }

            // insert settlement into DB
            // transfer fiat funds from crypto-currency receiver to holding account
            // mark settlement as pending ethereum transaction
            // perform ethereum transaction
            // 10 check ethereum transaction has been confirmed, and do one API lookup
            // 11 mark settlement as ethereum confirmed, pending fiat fund transfer
            // 12 transfer fiat funds from holding account to fiat currency receiver
            // 13 mark settlement as finished
        unimplemented!()
    }

    fn _get_order_account(&mut self, user_id: Id, order: &Order) -> Result<Account, CambioError> {
        let q = repository::UserClause::Id(user_id);
        let user_match = try!(self.user_repo.read(&q)).pop();
        if let None = user_match  {
            return Err(CambioError::not_found_search("Cannot find the user who made part of this order", 
                "User id was None"));
        }
        let user = user_match.unwrap();
        let accounts = try!(self.account_repository.get_accounts_for_user(user.id.unwrap()));
        for account in accounts.into_iter() {
            if order.buy_asset_type == account.asset_type && 
            order.buy_asset_denom == account.asset_denom && 
            account.account_business_type == AccountBusinessType::UserCashWallet {
                    return Ok(account)
            }
        }
        Err(CambioError::not_found_search("Could not found an account to credit for this order", 
            "No account matches order asset type and wallet business type"))
    }

    pub fn get_order_owner(&mut self, order_id: Id) -> Result<Option<User>, CambioError> {
        let user_result = self.db_helper.query(
            USER_FROM_ORDER,
            &[&order_id],
        );
        let mut users = try!(user_result);
        Ok(users.pop())
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

const CANCEL_ORDER_SQL: &'static str = "
    UPDATE asset_order SET status = 'user_cancelled' 
    WHERE status = 'active' AND expires_at > (now() at time zone 'utc') AND id = $1";

const UPDATE_ORDERS_EXPIRED_SQL: &'static str = "
    UPDATE asset_orders SET status = 'expires' 
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


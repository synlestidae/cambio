use chrono::prelude::*;
use domain::CurrencyCode;
use api::OrderRequest;
use db::{CambioError, PostgresHelper};
use domain;
use domain::*;
use postgres::GenericConnection;
use repository::{Creatable, Readable, Updateable};
use web3::types::U256;
use services::LedgerService;

pub struct OrderService {
    ledger_service: LedgerService
}

const ORDER_TIME_MINUTES: i64 = 10;

impl OrderService {
    pub fn new() -> Self {
        Self {
            ledger_service: LedgerService::new()
        }
    }

    pub fn place_order<C: GenericConnection>(
        &self,
        db: &mut C,
        user_id: UserId,
        order_request: &OrderRequest) -> Result<Order, CambioError> {
        let mut tx = db.transaction()?;

        let user: User = user_id.get(&mut tx)?;
        let owner_id = user.owner_id.unwrap();
        let accounts = AccountSet::from(owner_id.get_vec(&mut tx)?)?;

        // hold the money on behalf of the user
        self.ledger_service.transfer_money_positive_deduction(&mut tx, 
            accounts.nzd_wallet(), 
            accounts.nzd_hold(),
            CurrencyCode::NZD.asset_type(),
            order_request.amount_fiat
        )?;

        let created_order = order_request.clone().into_order(owner_id).create(&mut tx)?;
        tx.commit()?;
        Ok(created_order)
    }

    fn get_order_expiry(&self) -> DateTime<Utc> {
        use time::Duration;
        let now = Utc::now();
        now + Duration::minutes(ORDER_TIME_MINUTES)
    }

    pub fn cancel_order<C: GenericConnection>(
        &self,
        db: &mut C,
        order_id: OrderId,
    ) -> Result<Order, CambioError> {
        let mut order: Order = try!(order_id.get(db)); //self.order_repo.read(&order_clause);
        if order.status == domain::OrderStatus::Active {
            order.status = domain::OrderStatus::Deleted;
            Ok(try!(order.update(db)))
        } else {
            Err(CambioError::not_permitted(
                "Can only cancel an active order",
                "Can only change order status if status = 'active'",
            ))
        }
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

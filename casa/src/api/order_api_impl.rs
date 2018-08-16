use api::OrderChanges;
use api::utils;
use api;
use chrono::prelude::*;
use db::CambioError;
use db::ConnectionSource;
use db::PostgresHelper;
use db;
use domain::SettlementCriteria;
use domain::Order;
use domain::OrderChange;
use domain::AssetType;
use domain::OwnerId;
use api::OrderRequest;
use domain;
use hyper::mime::Mime;
use iron;
use postgres::GenericConnection;
use postgres::transaction::Transaction;
use repository::Creatable;
use repository::Readable;
use repository::RepoRead;
use repository;
use serde_json;
use services::OrderService;
use services::SettlementService;
use services::LedgerService;

pub struct OrderApiImpl<C: GenericConnection> {
    db: C,
    order_service: OrderService,
    settlement_service: SettlementService,
    ledger_service: LedgerService
}

impl<C: GenericConnection> OrderApiImpl<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db,
            order_service: OrderService::new(),
            settlement_service: SettlementService::new(),
            ledger_service: LedgerService::new()
        }
    }

    pub fn get_active_orders(&mut self) -> iron::Response {
        let order_result = domain::All.get_vec(&mut self.db);
        match order_result {
            Ok(orders) => utils::to_response(Ok(orders)),
            err => utils::to_response(err),
        }
    }

    pub fn get_changed_orders(&mut self, datetime: &DateTime<Utc>) -> Result<OrderChanges, db::CambioError> {
        const SQL: &'static str = "
            SELECT asset_order.*, order_changes.*, order_changes.id as order_change_id
            FROM asset_order
            JOIN order_changes ON order_changes.order_id = asset_order.id
            WHERE order_changes.changed_at > $1
            ORDER BY order_changes.changed_at";
        let mut orders = Vec::new();
        let mut changes: Vec<OrderChange> = Vec::new();
        for row in self.db.query(SQL, &[&datetime.naive_utc()])?.iter() {
            orders.push(db::TryFromRow::try_from_row(&row)?);
            changes.push(db::TryFromRow::try_from_row(&row)?);
        }
        Ok(OrderChanges::new(datetime.clone(), changes, orders))
    }

    pub fn get_user_orders(&mut self, user: &domain::User) -> iron::Response {
        let owner_id = match user.owner_id {
            Some(ref o) => o,
            None => {
                return db::CambioError::missing_field(
                    "User",
                    "User object is missing `owner_id` field",
                ).into()
            }
        };
        let orders: Result<Vec<Order>, _> = owner_id.get_vec(&mut self.db);
        match orders {
            Ok(orders) => utils::to_response(Ok(orders)),
            Err(err) => err.into(), 
        }
    }

    pub fn post_new_order(
        &mut self,
        user: &domain::User,
        order_request: &api::OrderRequest,
    ) -> Result<Order, CambioError> {
        let mut db_tx = self.db.transaction()?;
        let order = self.create_order(&mut db_tx, &order_request, &user.email_address)?;
        let eth_account_id = order_request.address.get(&mut db_tx)?.id.unwrap(); // TODO VERY VERY STUPID
        let criteria = if order.is_buy() {
            SettlementCriteria::criteria_for_buy(order.id.unwrap(), 
                order_request.minutes_to_settle,
                order_request.pledge,
                eth_account_id)
        } else {
            SettlementCriteria::criteria_for_sell(order.id.unwrap(), 
                order_request.minutes_to_settle,
                order_request.pledge,
                eth_account_id)
        };
        criteria.create(&mut db_tx)?;
        db_tx.commit()?;
        Ok(order)
    }

    pub fn complete_sell_order(&mut self, 
        user: &domain::User, 
        completion_request: &api::OrderCompletionRequest) -> Result<(), CambioError> {
        self.complete_buy_order(user, completion_request)?;
        Ok(())
    }


    pub fn complete_buy_order(&mut self, user: &domain::User, completion_request: &api::OrderCompletionRequest) -> Result<(), CambioError> {
        let mut tx = self.db.transaction()?;
        let counterparty_order: Order = completion_request.counterparty_order.get(&mut tx)?;
        self.check_order(user.owner_id.unwrap(), 
            &counterparty_order, 
            &completion_request.order_request)?;
        self.settlement_service.init_settlement(&mut tx,
            user,
            &counterparty_order,
            &completion_request.order_request
        )?;
        tx.commit()?;
        Ok(())
    }

    fn check_order(&self, owner_id: OwnerId, counterparty_order: &Order, request: &OrderRequest) -> Result<(), CambioError> {
        if counterparty_order.is_expired() || !counterparty_order.is_active() {
            panic!("This order is no longer available for settlement.");
        }
        if counterparty_order.owner_id == owner_id {
            panic!("Refuse to trade orders from same person!");
        }
        //let new_order = trade_request.order_request.clone().into_order(owner_id);
        if !request.is_fair(&counterparty_order) {
            panic!("Offer is not fair");
        }
        Ok(()) 
    }

    fn create_order<Co: GenericConnection>(
        &self,
        db: &mut Co,
        order: &api::OrderRequest,
        email_address: &str,
    ) -> Result<domain::Order, CambioError> {
        let mut tx = db.transaction()?;
        info!("User with email {} placing {} of fiat={:?}, crypto={:?}", 
              email_address, 
              if order.is_buy { "BUY" } else { "SELL" }, 
              order.amount_fiat, 
              order.amount_crypto);
        let user: domain::User = Readable::get(email_address, &mut tx)?;
        let user_id = user.id.unwrap();
        let placed_order = self.order_service.place_order(&mut tx, user_id, order)?;
        tx.commit()?;
        Ok(placed_order)
    }
}

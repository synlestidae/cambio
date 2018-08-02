use api::OrderChanges;
use api::utils;
use api;
use chrono::prelude::*;
use db::CambioError;
use db::ConnectionSource;
use db::PostgresHelper;
use db;
use domain::Order;
use domain::OrderChange;
use domain::OwnerId;
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

pub struct OrderApiImpl<C: GenericConnection> {
    db: C,
    order_service: OrderService,
    settlement_service: SettlementService
}

impl<C: GenericConnection> OrderApiImpl<C> {
    pub fn new(db: C) -> Self {
        let eth_path = "http://localhost:303030";
        Self {
            db: db,
            order_service: OrderService::new(),
            settlement_service: SettlementService::new()
        }
    }

    pub fn create_order<D: GenericConnection>(
        &self,
        db: &mut D,
        order: &api::OrderRequest,
        email_address: &str,
    ) -> Result<domain::Order, CambioError> {
        let user: domain::User = Readable::get(email_address, db)?;
        let user_id = user.id.unwrap();
        let placed_order = self.order_service.place_order(db, user_id, order)?;
        Ok(placed_order)
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
        order: &api::OrderRequest,
    ) -> iron::Response {
        let unauth_resp = api::ApiError::from(db::CambioError::unauthorised());
        let mut db_tx = self.db.transaction().unwrap();
        match self.create_order(&mut db_tx, &order, &user.email_address) {
            Ok(order) => {
                db_tx.commit();
                utils::to_response(Ok(order))
            }
            Err(err) => return err.into(),
        }
    }

    pub fn complete_sell_order(&mut self, user: &domain::User, trade_request: &api::TradeRequest) -> Result<(), CambioError> {
        let owner_id = user.owner_id.unwrap();
        let counterparty_order: Order = trade_request.counterparty_order.get(&mut self.db)?;
        let trade_request_order = trade_request.order_request.clone().into_order(owner_id);
        if !counterparty_order.is_buy() {
            self.check_order(owner_id, &counterparty_order, &trade_request_order)?;
        } else {
            return Err(CambioError::not_permitted(
                "A sell order must be completed with a buy order", 
                "Counterparty order retrieved in complete_sell_order is a buy order"));
        }
        self.settlement_service.init_settlement_of_sell(&mut self.db,
            &counterparty_order,
            &user,
            &trade_request.order_request)?;

        Ok(())
    }


    pub fn post_buy_order(&mut self, user: &domain::User, trade_request: &api::CryptoTradeRequest) -> Result<(), CambioError> {
        unimplemented!()
    }

    fn check_order(&self, owner_id: OwnerId, counterparty_order: &Order, request_order: &Order) -> Result<(), CambioError> {
        if counterparty_order.is_expired() || !counterparty_order.is_active() {
            panic!("This order is no longer available for settlement.");
        }
        if counterparty_order.owner_id == owner_id {
            panic!("Refuse to trade orders from same person!");
        }
        //let new_order = trade_request.order_request.clone().into_order(owner_id);
        if !request_order.is_fair(&counterparty_order) {
            panic!("Offer is not fair");
        }
        Ok(()) 
    }
}

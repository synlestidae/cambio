use api;
use chrono::prelude::*;
use api::utils;
use db;
use db::CambioError;
use db::ConnectionSource;
use db::PostgresHelper;
use domain;
use domain::OrderChange;
use api::OrderChanges;
use domain::Order;
use hyper::mime::Mime;
use iron;
use postgres::transaction::Transaction;
use postgres::GenericConnection;
use repository;
use repository::Creatable;
use repository::Readable;
use repository::RepoRead;
use serde_json;
use services::OrderService;
use services::SettlementService;

pub struct OrderApiImpl<C: GenericConnection> {
    db: C,
    order_service: OrderService,
}

impl<C: GenericConnection> OrderApiImpl<C> {
    pub fn new(db: C) -> Self {
        let eth_path = "http://localhost:303030";
        Self {
            db: db,
            order_service: OrderService::new(),
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

    pub fn post_buy_order(&mut self, user: &domain::User, trade_request: &api::OrderBuy) -> Result<(), CambioError> {
        let settlement_service = SettlementService::new();
        let owner_id = user.owner_id.unwrap();
        
        let counterparty_order: Order = trade_request.order_id.get(&mut self.db)?;
        if counterparty_order.is_expired() || !counterparty_order.is_active() {
            panic!("This order is no longer available for settlement.");
        }
        if counterparty_order.owner_id == owner_id {
            panic!("Refuse to trade orders from same person!");
        }
        let trade_request_order = trade_request.order_request.clone();
        let new_order = trade_request.order_request.clone().into_order(owner_id);
        if !new_order.is_fair(&counterparty_order) {
            panic!("Offer is not fair");
        }

        if counterparty_order.is_buy() {
            settlement_service.init_settlement_of_buy(&mut self.db, 
                &counterparty_order,
                &user, 
                &trade_request_order)?;
        } else {
            settlement_service.init_settlement_of_sell(&mut self.db,
                &counterparty_order,
                &user,
                &trade_request_order)?;
        }

        unimplemented!("We are going to change how buying an order works");
        /*info!(
            "User {} is completing order {:?}",
            user.email_address, order.order_id
        );
        let email_address = &user.email_address;
        let mut db_tx = self.db.transaction().unwrap();
        let response = {
            let counterparty_order: Order = order.order_id.get(&mut db_tx).unwrap();
            info!("Found order {:?}", order.order_id);
            // check the orders are valid and compatible with each other
            let unfair_err = db::CambioError::unfair_operation(
                "The order you chose is either incompatible or no longer active",
                "Target order.is_fair() returned false",
            );
            if !counterparty_order.is_active() {
                info!(
                    "Order {:?} has expired, can't complete settlement",
                    counterparty_order.id
                );
                let err = db::CambioError::not_permitted(
                    "The order you chose is expired or no longer active",
                    "Target order is expired or is not active",
                );
                return api::ApiError::from(err).into();
            }*/
            //let request_copy = order.order_request.clone();
            /*if counterparty_order.sell_asset_type != request_copy.get_buy_asset_type() {
                info!(
                    "Target order {:?} has sell type {:?}, but request buy type is {:?}",
                    counterparty_order.id, counterparty_order.sell_asset_type, request_copy.get_buy_asset_type()
                );
                return db::CambioError::unfair_operation(
                    "Request sell_asset_type does not match target buy_asset_type",
                    "Target order.is_fair() returned false",
                ).into();
            }
            info!("Checking that the buy and sell asset types match");
            if counterparty_order.buy_asset_type != request_copy.get_sell_asset_type() {
                return db::CambioError::unfair_operation(
                    "Request buy_asset_type does not match target sell_asset_type",
                    "Target order.is_fair() returned false",
                ).into();
            }
            if counterparty_order.sell_asset_units != request_copy.get_buy_asset_units() as i64 {
                return db::CambioError::unfair_operation(
                    "Request sell_asset_units does not match target buy_asset_units",
                    "Target order.is_fair() returned false",
                ).into();
            }
            if counterparty_order.buy_asset_units != request_copy.get_sell_asset_units() as i64{
                return db::CambioError::unfair_operation(
                    "Request sell_asset_units does not match target buy_asset_units",
                    "Target order.is_fair() returned false",
                ).into();
            }*/
            /*
            info!("Creating order from request for order {:?}", order.order_id);
            let our_order =
                match self.create_order(&mut db_tx, &order.order_request, &email_address) {
                    Ok(o) => o,
                    Err(resp) => return resp,
                };

            info!(
                "Creating a settlement between orders {:?} and {:?}",
                order.order_id, our_order.id
            );
            // save the settlement
            let settlement = if counterparty_order.buy_asset_type.is_crypto() {
                // counterparty_order is the buying order
                domain::OrderSettlement::from(user.id.unwrap(), &counterparty_order, &our_order)
            } else {
                domain::OrderSettlement::from(user.id.unwrap(), &our_order, &counterparty_order)
            };

            let settlement_result = settlement.create(&mut db_tx);

            // generate the receipt
            let response = match settlement_result {
                Ok(settlement) => {
                    db_tx.commit();
                    info!("Settlement creation was successful");
                    let response_json = serde_json::to_string(&settlement).unwrap();
                    let content_type = "application/json".parse::<Mime>().unwrap();
                    iron::Response::with((iron::status::Ok, response_json, content_type))
                }
                Err(err) => return api::ApiError::from(err).into(),
            };
            response
        };
        response*/
    }
}

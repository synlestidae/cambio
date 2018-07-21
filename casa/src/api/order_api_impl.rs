use api;
use api::utils;
use db;
use db::ConnectionSource;
use db::PostgresHelper;
use domain;
use domain::Order;
use hyper::mime::Mime;
use iron;
use postgres::transaction::Transaction;
use postgres::GenericConnection;
use repositories;
use repository;
use repository::Creatable;
use repository::Readable;
use repository::RepoRead;
use serde_json;
use services;

pub struct OrderApiImpl<C: GenericConnection> {
    db: C,
    order_service: services::OrderService,
}

impl<C: GenericConnection> OrderApiImpl<C> {
    pub fn new(db: C) -> Self {
        let eth_path = "http://localhost:303030";
        Self {
            db: db,
            order_service: services::OrderService::new(),
        }
    }

    fn create_order<D: GenericConnection>(
        &self,
        db: &mut D,
        order: &api::OrderRequest,
        email_address: &str,
    ) -> Result<domain::Order, iron::Response> {
        let order_result = self.order_service.place_order(
            db,
            email_address,
            &order.unique_id,
            order.sell_asset_units as u64,
            order.sell_asset_type,
            order.buy_asset_units as u64,
            order.buy_asset_type,
            order.max_wei,
        );
        match order_result {
            Ok(result) => Ok(result),
            err => Err(utils::to_response(err)),
        }
    }
}

impl<C: GenericConnection> api::OrderApiTrait for api::OrderApiImpl<C> {
    fn get_active_orders(&mut self) -> iron::Response {
        let order_result = domain::All.get_vec(&mut self.db);
        match order_result {
            Ok(orders) => utils::to_response(Ok(orders)),
            err => utils::to_response(err),
        }
    }

    fn get_user_orders(&mut self, user: &domain::User) -> iron::Response {
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
            Err(err) => err.into(), //api::ApiError::from(err).into()
        }
    }

    fn post_new_order(&mut self, user: &domain::User, order: &api::OrderRequest) -> iron::Response {
        let unauth_resp = api::ApiError::from(db::CambioError::unauthorised());
        if order.sell_asset_type.is_crypto() && order.max_wei.is_none() {
            const WEI_MSG: &'static str = "To sell Ethereum, please specify your transaction cost";
            return api::ApiError::missing_field_or_param(WEI_MSG).into();
        }
        let mut db_tx = self.db.transaction().unwrap();
        match self.create_order(&mut db_tx, &order, &user.email_address) {
            Ok(order) => {
                db_tx.commit();
                utils::to_response(Ok(order))
            },
            Err(err_resp) => return err_resp,
        }
    }

    fn post_buy_order(&mut self, user: &domain::User, order: &api::OrderBuy) -> iron::Response {
        info!(
            "User {} is completing order {:?}",
            user.email_address, order.order_id
        );
        let email_address = &user.email_address;
        let mut db_tx = self.db.transaction().unwrap();
        let response = {
            let target_order: Order = order.order_id.get(&mut db_tx).unwrap();
            info!("Found order {:?}", order.order_id);
            // check the orders are valid and compatible with each other
            let unfair_err = db::CambioError::unfair_operation(
                "The order you chose is either incompatible or no longer active",
                "Target order.is_fair() returned false",
            );
            if !target_order.is_active() {
                info!(
                    "Order {:?} has expired, can't complete settlement",
                    target_order.id
                );
                let err = db::CambioError::not_permitted(
                    "The order you chose is expired or no longer active",
                    "Target order is expired or is not active",
                );
                return api::ApiError::from(err).into();
            }
            let request_copy = order.order_request.clone();
            if target_order.sell_asset_type != request_copy.buy_asset_type {
                info!(
                    "Target order {:?} has sell type {:?}, but request buy type is {:?}",
                    target_order.id, target_order.sell_asset_type, request_copy.buy_asset_type
                );
                return db::CambioError::unfair_operation(
                    "Request sell_asset_type does not match target buy_asset_type",
                    "Target order.is_fair() returned false",
                ).into();
            }
            info!("Checking that the buy and sell asset types match");
            if target_order.buy_asset_type != request_copy.sell_asset_type {
                return db::CambioError::unfair_operation(
                    "Request buy_asset_type does not match target sell_asset_type",
                    "Target order.is_fair() returned false",
                ).into();
            }
            if target_order.sell_asset_units != request_copy.buy_asset_units {
                return db::CambioError::unfair_operation(
                    "Request sell_asset_units does not match target buy_asset_units",
                    "Target order.is_fair() returned false",
                ).into();
            }
            if target_order.buy_asset_units != request_copy.sell_asset_units {
                return db::CambioError::unfair_operation(
                    "Request sell_asset_units does not match target buy_asset_units",
                    "Target order.is_fair() returned false",
                ).into();
            }

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
            let settlement = if target_order.buy_asset_type.is_crypto() {
                // target_order is the buying order
                domain::OrderSettlement::from(user.id.unwrap(), &target_order, &our_order)
            } else {
                domain::OrderSettlement::from(user.id.unwrap(), &our_order, &target_order)
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
        response
    }
}

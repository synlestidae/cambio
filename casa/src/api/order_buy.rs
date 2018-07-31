use api::OrderRequest;
use chrono::prelude::*;
use chrono::NaiveDate;
use domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBuy {
    pub order_id: domain::OrderId,
    pub order_request: OrderRequest,
}

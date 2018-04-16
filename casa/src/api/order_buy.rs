use chrono::NaiveDate;
use chrono::prelude::*;
use domain;
use api::OrderRequest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderBuy {
    pub unique_id: String,
    pub order_id: domain::Id,
    pub order_request: OrderRequest
}

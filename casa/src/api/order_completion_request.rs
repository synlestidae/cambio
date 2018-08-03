use api::OrderRequest;
use chrono::prelude::*;
use chrono::NaiveDate;
use domain;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCompletionRequest {
    pub counterparty_order: domain::OrderId,
    pub order_request: OrderRequest
}


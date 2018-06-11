use chrono::prelude::*;
use chrono::NaiveDate;
use domain;
use std::convert::Into;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub unique_id: String,
    pub sell_asset_type: domain::AssetType,
    pub sell_asset_units: i64,
    pub buy_asset_type: domain::AssetType,
    pub buy_asset_units: i64,
    pub expires_at: DateTime<Utc>,
}

use chrono::NaiveDate;
use chrono::prelude::*;
use domain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub unique_id: String,
    pub sell_asset_type: domain::AssetType,
    pub sell_asset_denom: domain::Denom,
    pub sell_asset_units: u64,
    pub buy_asset_type: domain::AssetType,
    pub buy_asset_denom: domain::Denom,
    pub buy_asset_units: u64,
    pub expires_at: DateTime<Utc>,
}

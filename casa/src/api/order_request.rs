use chrono::NaiveDate;
use chrono::prelude::*;
use domain;
use std::convert::Into;

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

/*impl Into<domain::Order> for OrderRequest {
    fn into(self) -> domain::Order {
        domain::Order {
            id: None,
            owner_id: None,
            unique_id: self.unique_id,
            sell_asset_type: self.sell_asset_type,
            sell_asset_denom: self.sell_asset_denom,
            sell_asset_units: self.sell_asset_units as u64,
            buy_asset_type: self.buy_asset_type,
            buy_asset_denom: self.buy_asset_denom,
            buy_asset_units: self.buy_asset_units as u64,
            expires_at: self.expires_at,
            status: domain::OrderStatus::Active
        }
    }
}*/

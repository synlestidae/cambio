use domain::asset_type::AssetType;
use domain::denom::Denom;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: Option<u64>,
    pub sell_asset_units: u64,
    pub buy_asset_units: u64,
    pub sell_asset_type: AssetType,
    pub sell_asset_denom: Denom,
    pub buy_asset_type: AssetType,
    pub buy_asset_denom: Denom,
    pub expires_at: DateTime<Utc>
}

impl Order {
    pub fn can_exchange(&self, other_order: &Order) -> bool {
        return self.buy_asset_type == other_order.sell_asset_type && 
            self.buy_asset_denom == other_order.sell_asset_denom;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    pub splittable: bool,
}

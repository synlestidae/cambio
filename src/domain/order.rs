use serde_derive;
use domain::asset_type::AssetType;
use domain::denom::Denom;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub sell_asset_units: u64,
    pub buy_asset_units: u64,
    pub sell_asset_type: AssetType,
    pub sell_asset_denom: Denom,
    pub buy_asset_type: AssetType,
    pub buy_asset_denom: Denom,
    pub debit_account: u32,
    pub credit_account: u32,
    pub order_info: OrderInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderInfo {
    pub splittable: bool
}

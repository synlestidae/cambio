use domain::Decimal;
use domain::AssetType;
use chrono::prelude::*;
use web3::types::U256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub unique_id: String,
    pub amountFiat: Decimal,
    pub amountCrypto: U256,
    pub isBuy: bool,
    pub minutesActive: u32,
    #[serde(default)]
    pub max_wei: Option<U256>,
}

impl OrderRequest {
    pub fn get_sell_asset_units(&self) -> u64 {
        unimplemented!()
    }

    pub fn get_buy_asset_units(&self) -> u64 {
        unimplemented!()
    }

    pub fn get_sell_asset_type(&self) -> AssetType {
        unimplemented!()
    }

    pub fn get_buy_asset_type(&self) -> AssetType {
        unimplemented!()
    }

    pub fn get_expiry(&self) -> DateTime<Utc> {
        let now = Utc::now();
        unimplemented!();
    }
}

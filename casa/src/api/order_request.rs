use domain::Decimal;
use chrono::Duration;
use domain::AssetType;
use chrono::prelude::*;
use web3::types::U256;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderRequest {
    pub unique_id: String,
    pub amount_fiat: Decimal,
    pub amount_crypto: U256,
    pub is_buy: bool,
    pub minutes_active: u32,
    #[serde(default)]
    pub max_wei: Option<U256>,
}

impl OrderRequest {
    pub fn get_sell_asset_units(&self) -> u64 {
        if self.is_buy {
            self.amount_fiat.to_cents() as u64
        } else {
            self.amount_crypto.low_u64()
        }
    }

    pub fn get_buy_asset_units(&self) -> u64 {
        if !self.is_buy {
            self.amount_fiat.to_cents() as u64
        } else {
            self.amount_crypto.low_u64()
        }
    }

    pub fn get_sell_asset_type(&self) -> AssetType {
        if self.is_buy {
            AssetType::NZD
        } else {
            AssetType::ETH
        }
    }

    pub fn get_buy_asset_type(&self) -> AssetType {
        if !self.is_buy {
            AssetType::NZD
        } else {
            AssetType::ETH
        }
    }

    pub fn get_expiry(&self) -> DateTime<Utc> {
        Utc::now() + Duration::minutes(self.minutes_active as i64)
    }
}

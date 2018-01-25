use domain::{AssetType, Denom, Id, OrderStatus};
use chrono::Duration;
use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use chrono::{DateTime, Utc};
use std;
use postgres::rows::Row;
use postgres;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Order {
    #[column_id(order_id)]
    pub id: Option<Id>,
    pub unique_id: String,
    pub sell_asset_units: i64,
    pub buy_asset_units: i64,
    #[column_id(sell_asset_code)]
    pub sell_asset_type: AssetType,
    pub sell_asset_denom: Denom,
    #[column_id(buy_asset_code)]
    pub buy_asset_type: AssetType,
    pub buy_asset_denom: Denom,
    pub expires_at: DateTime<Utc>,
    pub status: OrderStatus
}

impl Order {
    pub fn can_exchange(&self, other_order: &Order) -> bool {
        return self.buy_asset_type == other_order.sell_asset_type &&
            self.buy_asset_denom == other_order.sell_asset_denom;
    }
}

use domain;
use domain::{AssetType, Denom, Id, OrderStatus};
use chrono::Duration;
use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use chrono::{DateTime, Utc};
use std;
use postgres::rows::Row;
use postgres;
use rand;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Order {
    #[column_id(order_id)]
    pub id: Option<Id>,
    pub owner_id: Id,
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
    pub fn buy_szabo(owner: Id, buy: u64, nzd_cents: u64, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        println!("burns order expires at {}", expiry);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: nzd_cents as i64,
            buy_asset_units: buy as i64,
            sell_asset_type: domain::AssetType::NZD,
            sell_asset_denom: domain::Denom::Cent,
            buy_asset_type: domain::AssetType::ETH,
            buy_asset_denom: domain::Denom::Szabo,
            expires_at: expiry,
            status: domain::OrderStatus::Active
        }
    }


    pub fn sell_szabo(owner: Id, buy_cents: u64, szabo: u64, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: szabo as i64,
            buy_asset_units: buy_cents as i64,
            sell_asset_type: domain::AssetType::ETH,
            sell_asset_denom: domain::Denom::Szabo,
            buy_asset_type: domain::AssetType::NZD,
            buy_asset_denom: domain::Denom::Cent,
            expires_at: expiry,
            status: domain::OrderStatus::Active
        }
    }

    pub fn can_exchange(&self, other_order: &Order) -> bool {
        return self.buy_asset_type == other_order.sell_asset_type &&
            self.buy_asset_denom == other_order.sell_asset_denom;
    }

    pub fn is_fair(&self, other_order: &Order) -> bool {
        unimplemented!()
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now() 
    }
}


fn random_string() -> String {
    let mut token = String::new();
    for _ in (0..32) {
            token.push(rand::random::<char>() as char);
    }
    token
}

use chrono::prelude::*;
use chrono::Duration;
use chrono::{DateTime, Utc};
use db::{CambioError, PostgresHelper, TryFromRow, TryFromRowError};
use domain;
use domain::{AssetType, Id, OrderId, OrderStatus, OwnerId, User};
use postgres;
use postgres::rows::Row;
use rand;
use repository::Readable;
use std;
use web3::types::U256;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Order {
    #[column_id(order_id)]
    pub id: Option<OrderId>,
    pub owner_id: OwnerId,
    pub unique_id: String,
    pub sell_asset_type: AssetType,
    pub sell_asset_units: i64,
    pub buy_asset_type: AssetType,
    pub buy_asset_units: i64,
    pub expires_at: DateTime<Utc>,
    pub status: OrderStatus,
    pub max_wei: Option<U256>,
}

impl Order {
    pub fn buy_szabo(owner: OwnerId, buy: u64, nzd_cents: u32, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: nzd_cents as i64,
            buy_asset_units: buy as i64,
            sell_asset_type: domain::AssetType::NZD,
            buy_asset_type: domain::AssetType::ETH,
            expires_at: expiry,
            status: domain::OrderStatus::Active,
            max_wei: None,
        }
    }

    pub fn sell_szabo(owner: OwnerId, buy_cents: u32, szabo: u64, ttl_minutes: u32) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::minutes(ttl_minutes as i64);

        Order {
            id: None,
            owner_id: owner,
            unique_id: random_string(),
            sell_asset_units: szabo as i64,
            buy_asset_units: buy_cents as i64,
            sell_asset_type: domain::AssetType::ETH,
            buy_asset_type: domain::AssetType::NZD,
            expires_at: expiry,
            status: domain::OrderStatus::Active,
            max_wei: None,
        }
    }

    pub fn is_fair(&self, other: &Order) -> bool {
        return self.sell_asset_type == other.buy_asset_type
            && self.buy_asset_type == other.sell_asset_type
            && self.sell_asset_units == other.buy_asset_units
            && self.buy_asset_units == other.sell_asset_units;
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    pub fn is_active(&self) -> bool {
        !self.is_expired() && self.status == domain::OrderStatus::Active
    }
}

#[derive(TryFromRow)]
struct OrderRow {
    #[column_id(order_id)]
    pub id: Option<OrderId>,
    pub owner_id: OwnerId,
    pub unique_id: String,
    pub sell_asset_type: AssetType,
    pub sell_asset_units: i64,
    pub buy_asset_type: AssetType,
    pub buy_asset_units: i64,
    pub expires_at: NaiveDateTime,//DateTime<Utc>,
    pub status: OrderStatus,
    pub max_wei: Option<Vec<u8>>,
}

impl TryFromRow for Order {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let order_row: OrderRow = try!(OrderRow::try_from_row(row));
        let wei: Option<U256> = order_row.max_wei.map(|w| {
            let mut array: [u8; 32] = [0; 32];
            array.copy_from_slice(&w);
            U256::from(array)
        });
        let expires_at = DateTime::from_utc(order_row.expires_at, Utc);
        Ok(Order {
            id: order_row.id,
            owner_id: order_row.owner_id,
            unique_id: order_row.unique_id,
            sell_asset_type: order_row.sell_asset_type,
            sell_asset_units: order_row.sell_asset_units,
            buy_asset_type: order_row.buy_asset_type,
            buy_asset_units: order_row.buy_asset_units,
            expires_at: expires_at,
            status: order_row.status,
            max_wei: wei,
        })
    }
}

fn random_string() -> String {
    let mut token = String::new();
    for _ in (0..32) {
        token.push(rand::random::<char>() as char);
    }
    token
}

use chrono::Duration;
use chrono::prelude::*;
use chrono::{DateTime, Utc};
use db::{CambioError, PostgresHelper, TryFromRow, TryFromRowError};
use domain::*;
use postgres::rows::Row;
use postgres;
use rand;
use repository::Readable;
use std;
use web3::types::U256;

#[derive(TryFromRow, Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Order {
    #[column_id(order_id)]
    pub id: Option<OrderId>,
    #[serde(skip_serializing)]
    pub owner_id: OwnerId,
    pub unique_id: String,
    pub amount_fiat: Decimal,
    pub amount_crypto: BigInteger,
    pub trade_type: TradeType,
    pub fiat_type: CurrencyCode,
    pub crypto_type: CryptoType,
    pub expires_at: DateTime<Utc>,
    pub status: OrderStatus
}

impl Order {
    pub fn is_fair(&self, other: &Order) -> bool {
        return self.trade_type.is_compatible(&other.trade_type) &&
            self.amount_fiat == other.amount_fiat &&
            self.amount_crypto == other.amount_crypto &&
            self.fiat_type == other.fiat_type &&
            self.crypto_type == other.crypto_type;
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    pub fn is_active(&self) -> bool {
        !self.is_expired() && self.status == OrderStatus::Active
    }

    pub fn is_buy(&self) -> bool {
        self.trade_type == TradeType::BuyCrypto
    }

    pub fn begin_settling(&mut self) {
        if self.status == OrderStatus::Active {
            self.status = OrderStatus::Settling;
        }
    }
}

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

    pub fn mark_settled(&mut self) {
        self.status = OrderStatus::Settled;
    }
}

pub struct OrderBuilder {
    owner_id: OwnerId
}

impl OrderBuilder {
    pub fn new(owner_id: OwnerId) -> Self {
        Self {
            owner_id: owner_id
        }
    }

    pub fn trade_nzd_eth(self, fiat: Decimal, crypto: BigInteger) -> TradeOrderBuilder {
        self.trade(fiat, CurrencyCode::NZD, crypto, CryptoType::Ether)
    }

    pub fn trade(self, 
        amount_fiat: Decimal,
        fiat_type: CurrencyCode,
        amount_crypto: BigInteger,
        crypto_type: CryptoType) -> TradeOrderBuilder {
        TradeOrderBuilder {
            owner_id: self.owner_id,
            amount_fiat: amount_fiat,
            fiat_type: fiat_type,
            amount_crypto: amount_crypto,
            crypto_type: crypto_type
        }
    }
}

pub struct TradeOrderBuilder {
    owner_id: OwnerId,
    amount_fiat: Decimal,
    fiat_type: CurrencyCode,
    amount_crypto: BigInteger,
    crypto_type: CryptoType,
}

impl TradeOrderBuilder {
    fn order(self, expiry_minutes: u32, unique_id: &str) -> Order {
        let expiry = Utc::now() + Duration::minutes(expiry_minutes as i64);
        Order {
            id: None,
            owner_id: self.owner_id,
            unique_id: unique_id.to_string(),
            amount_fiat: self.amount_fiat,
            amount_crypto: self.amount_crypto,
            crypto_type: self.crypto_type,
            trade_type: TradeType::BuyCrypto,
            fiat_type: self.fiat_type,
            expires_at: expiry,
            status: OrderStatus::Active
        }
    }

    pub fn buy_fiat(self, expiry_minutes: u32, unique_id: &str) -> Order {
        self.sell_crypto(expiry_minutes, unique_id)
    }

    pub fn sell_fiat(self, expiry_minutes: u32, unique_id: &str) -> Order {
        self.buy_crypto(expiry_minutes, unique_id)
    }

    pub fn buy_crypto(self, expiry_minutes: u32, unique_id: &str) -> Order {
        let mut order = self.order(expiry_minutes, unique_id);
        order.trade_type = TradeType::BuyCrypto;
        order
    }

    pub fn sell_crypto(self, expiry_minutes: u32, unique_id: &str) -> Order {
        let mut order = self.order(expiry_minutes, unique_id);
        order.trade_type = TradeType::SellCrypto;
        order
    }
}

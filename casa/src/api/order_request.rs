use chrono::Duration;
use chrono::prelude::*;
use domain::BigInteger;
use domain::CryptoType;
use domain::ByteAddress;
use domain::CurrencyCode;
use domain::Decimal;
use domain::Order;
use domain::OrderStatus;
use domain::TradeType;
use domain::OwnerId;
use web3::types::U256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    pub unique_id: String,
    pub amount_fiat: Decimal,
    pub amount_crypto: BigInteger,
    pub is_buy: bool,
    pub minutes_active: u32,
    pub minutes_to_settle: u32,
    pub pledge: Decimal,
    pub address: ByteAddress
}

impl OrderRequest {
    pub fn is_fair(&self, other: &Order) -> bool {
        self.clone().into_order(OwnerId(0)).is_fair(other)
    }

    pub fn into_order(self, owner_id: OwnerId) -> Order {
        let trade_type = if self.is_buy {
            TradeType::BuyCrypto
        } else {
            TradeType::SellCrypto
        };
        Order {
            id: None,
            owner_id: owner_id,
            unique_id: self.unique_id,
            amount_fiat: self.amount_fiat,
            amount_crypto: self.amount_crypto,
            trade_type: trade_type,
            fiat_type: CurrencyCode::NZD,
            crypto_type: CryptoType::Ether,
            expires_at: Utc::now() + Duration::minutes(self.minutes_active as i64),
            status: OrderStatus::Active
        }
    }
}

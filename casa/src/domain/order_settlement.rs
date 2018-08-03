use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::{TryFromRow, TryFromRowError};
use domain::{Order, OrderSettlementId, SettlementStatus, UserId, EthAccountId};
use domain::OrderId;
use postgres;

#[derive(TryFromRow, Serialize, Debug)]
pub struct OrderSettlement {
    pub id: Option<OrderSettlementId>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: UserId,
    pub settlement_status: SettlementStatus,
    pub buying_fiat_id: OrderId,
    pub buying_crypto_id: OrderId,
    pub eth_account: EthAccountId
}

impl OrderSettlement {
    pub fn from(user_id: UserId, buy_order: &Order, sell_order: &Order, eth_account: EthAccountId) -> Self {
        OrderSettlement {
            id: None,
            started_at: Utc::now(),
            settled_at: None,
            starting_user: user_id,
            settlement_status: SettlementStatus::WaitingEth,
            buying_crypto_id: buy_order.id.unwrap(),
            buying_fiat_id: sell_order.id.unwrap(),
            eth_account: eth_account
        }
    }
}

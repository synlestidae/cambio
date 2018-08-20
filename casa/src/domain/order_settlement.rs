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
    pub settled_at: Option<NaiveDateTime>,
    pub starting_user: UserId,
    pub status: SettlementStatus,
    pub original_order: OrderId,
    pub settling_order: OrderId,
    pub eth_account: EthAccountId,
    pub settles_buy: bool
}

impl OrderSettlement {
    pub fn from(user_id: UserId, original_order: &Order, settling_order: &Order, eth_account_id: EthAccountId) -> Self {
        // TODO This method will check its input
        OrderSettlement {
            id: None,
            started_at: Utc::now(),
            settled_at: None,
            starting_user: user_id,
            status: SettlementStatus::WaitingEth,
            original_order: original_order.id.unwrap(),
            settling_order: settling_order.id.unwrap(),
            eth_account: eth_account_id.clone(),
            settles_buy: original_order.is_buy()
        }
    }

    pub fn mark_settled(&mut self) {
        self.status = SettlementStatus::Settled
    }

    pub fn can_proceed(&self) -> bool {
        self.status == SettlementStatus::WaitingEth
    }

    pub fn order_with_criteria(&self) -> OrderId {
        self.original_order
    }
}


use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::{TryFromRow, TryFromRowError};
use domain::{Order, OrderSettlementId, SettlementStatus, UserId};
use postgres;

#[derive(Serialize)]
pub struct OrderSettlement {
    pub id: Option<OrderSettlementId>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: UserId,
    pub settlement_status: SettlementStatus,
    pub buying_order: Order,
    pub selling_order: Order,
}

impl OrderSettlement {
    pub fn from(user_id: UserId, buy_order: &Order, sell_order: &Order) -> Self {
        OrderSettlement {
            id: None,
            started_at: Utc::now(),
            settled_at: None,
            starting_user: user_id,
            settlement_status: SettlementStatus::WaitingEthCredentials,
            buying_order: buy_order.clone(),
            selling_order: sell_order.clone(),
        }
    }
}

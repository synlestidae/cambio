use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use domain::{SettlementStatus, Order};
use db::{TryFromRow, TryFromRowError};

pub struct OrderSettlement {
    pub id: Option<i32>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub settlement_status: SettlementStatus,
    pub buying_order: Order,
    pub selling_order: Order
}

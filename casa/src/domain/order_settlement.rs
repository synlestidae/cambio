use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use domain::{SettlementStatus, Order};

pub struct OrderSettlement {
    pub id: Option<u64>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub settlement_status: SettlementStatus,
    pub buying_order: Order,
    pub selling_order: Order
}

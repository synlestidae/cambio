use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use domain::{SettlementStatus, Order, Id};
use db::{TryFromRow, TryFromRowError};
use postgres;

pub struct OrderSettlement {
    pub id: Option<Id>,
    pub started_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
    pub starting_user: Id,
    pub settlement_status: SettlementStatus,
    pub buying_order: Order,
    pub selling_order: Order,
}

use chrono::prelude::*;
use domain::OrderChange;
use domain::Order;

#[derive(Debug, Serialize)]
pub struct OrderChanges {
    pub from: DateTime<Utc>,
    pub to: DateTime<Utc>,
    pub changes: Vec<OrderChange>,
    pub affected_orders: Vec<Order>
}

impl OrderChanges {
    pub fn new(datetime: DateTime<Utc>, changes: Vec<OrderChange>, affected_orders: Vec<Order>) -> Self {
        OrderChanges {
            from: if changes.len() > 0 { changes[0].changed_at } else { datetime.clone() },
            to: if changes.len() > 0 { changes[changes.len() - 1].changed_at } else { datetime.clone() },
            changes: changes,
            affected_orders: affected_orders
        }
    }
}

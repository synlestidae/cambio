use domain::Id;
use domain::OrderId;
use chrono::prelude::*;
use postgres;
use db::TryFromRow;
use db::TryFromRowError;

#[derive(Debug, TryFromRow, Serialize)]
pub struct OrderChange {
    #[column_id(order_change_id)]
    pub id: Id,
    pub order_id: OrderId,
    pub changed_at: DateTime<Utc>,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>
}

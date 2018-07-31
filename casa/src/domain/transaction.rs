use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::TryFromRow;
use db::TryFromRowError;
use domain::{AssetType, BusinessEnds, Id, TransactionId, Decimal};
use postgres;
use postgres::rows::Row;

#[derive(Debug, TryFromRow, Clone, Serialize)]
pub struct Transaction {
    pub correspondence_id: Id,
    pub from_account: Id,
    pub to_account: Id,
    pub asset_type: AssetType,
    pub value: Decimal,
    pub transaction_time: DateTime<Utc>,
    pub accounting_period_id: Id,
    pub balance_to_account: i64,
}

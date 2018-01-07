use chrono::prelude::*;

pub struct Transaction {
    pub from: UserAccount,
    pub to: UserAccount,
    pub asset_type: AssetType,
    pub value: i64,
    pub transaction_time: DateTime<Utc>,
    pub accounting_period: i32,
    pub balance: i64,
    pub business_ends: BusinessEnds
}

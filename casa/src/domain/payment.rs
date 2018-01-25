use domain::{UniqueId, AssetType, Denom, PaymentVendor, PaymentMethod, Id};
use chrono::{DateTime, Utc};
use postgres;
use db::TryFromRowError;

pub struct Payment {
    pub id: Option<Id>,
    pub unique_id: UniqueId,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub datetime_payment_made: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub vendor: PaymentVendor,
    pub user_credit: i64,
    pub message: Option<String>,
}

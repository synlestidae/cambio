use domain::{UniqueId, AssetType, Denom, PaymentVendor, PaymentMethod};
use chrono::{DateTime, Utc};

pub struct Payment {
    pub unique_id: UniqueId,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub datetime_payment_made: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub vendor: PaymentVendor,
    pub user_credit: i64,
    pub message: Option<String>,
}


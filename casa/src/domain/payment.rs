use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use domain::{AssetType, Id, PaymentMethod, PaymentVendor, UniqueId};
use postgres;

#[derive(Clone, Debug, TryFromRow, Deserialize, Serialize)]
pub struct Payment {
    pub id: Option<Id>,
    pub unique_id: UniqueId,
    pub asset_type: AssetType,
    pub datetime_payment_made: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub vendor: PaymentVendor,
    pub user_credit: i64,
}

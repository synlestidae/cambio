use domain::{UniqueId, AssetType, Denom, PaymentVendor, PaymentMethod, Id};
use chrono::{DateTime, Utc};
use postgres;
use db::{TryFromRow, TryFromRowError};

#[derive(Clone, Debug)]
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

impl TryFromRow for Payment {
    fn try_from_row<'a>(row: &postgres::rows::Row<'a>) -> Result<Self, TryFromRowError> {
        unimplemented!()
    }
}

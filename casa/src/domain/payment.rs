use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use domain::{AssetType, Denom, Id, PaymentMethod, PaymentVendor, UniqueId};
use postgres;

#[derive(Clone, Debug, TryFromRow, Deserialize, Serialize)]
pub struct Payment {
    pub id: Option<Id>,
    pub unique_id: UniqueId,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub datetime_payment_made: DateTime<Utc>,
    pub payment_method: PaymentMethod,
    pub vendor: PaymentVendor,
    pub user_credit: i64,
}

/*impl TryFromRow for Payment {
    fn try_from_row<'a>(row: &postgres::rows::Row<'a>) -> Result<Self, TryFromRowError> {
        unimplemented!()
    }
}*/

/*struct PaymentRow {
    id: Id,
    vendor_name: domain::PaymentVendor, 
    payment_method: domain::PaymentMethod,
    datetime_payment_made: DateTime<Utc>,
    datetime_recorded: DateTime<Utc>,
    asset_code: domain::AssetType,
    asset_denom: domain::AssetDenom,
    units: i64 ,
    unique_id: String
}*/

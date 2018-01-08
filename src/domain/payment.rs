use domain::{UniqueId, AssetType, Denom, PaymentVendor};

pub struct Payment {
    pub unique_id: UniqueId,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub vendor: PaymentVendor,
    pub user_credit: i64,
    pub message: Option<String>
}

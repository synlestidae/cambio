use domain::{AssetType, Denom};

pub struct UserAccount {
    pub id: Option<u64>,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
}

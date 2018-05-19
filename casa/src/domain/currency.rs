use domain::{AssetType, Denom};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Currency {
    pub denom: Denom,
    pub asset_type: AssetType,
}

impl Currency {
    pub fn new(asset_type: AssetType, denom: Denom) -> Self {
        Self {
            asset_type: asset_type,
            denom: denom,
        }
    }
}

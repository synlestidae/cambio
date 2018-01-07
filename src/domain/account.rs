use domain::{AssetType, Denom, Id, AccountStatus, AccountBusinessType, AccountRole};

pub struct Account {
    pub id: Option<Id>,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub account_status: AccountStatus,
    pub account_business_type: AccountBusinessType,
    pub account_role: AccountRole
}

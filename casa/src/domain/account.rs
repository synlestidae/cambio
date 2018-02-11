use domain::{AssetType, Denom, Id, AccountStatus, AccountBusinessType, AccountRole, AccountType};
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;
use postgres;

#[derive(Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Account {
    #[column_id(account_id)]
    pub id: Option<Id>,
    #[column_id(user_id)]
    pub owner_user_id: Option<Id>,
    #[column_id(asset_code)]
    pub asset_type: AssetType,
    #[column_id(denom)]
    pub asset_denom: Denom,
    pub account_status: AccountStatus,
    pub account_type: AccountType,
    pub account_business_type: AccountBusinessType,
    pub account_role: AccountRole,
}

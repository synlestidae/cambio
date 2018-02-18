use domain::{AssetType, Denom, Id, AccountStatus, AccountBusinessType, AccountRole, AccountType};
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;
use postgres;

#[derive(Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Account {
    #[column_id(account_id)]
    pub id: Option<Id>,
    #[column_id(owner_id)]
    pub owner_user_id: Option<Id>,
    #[column_id(account_asset_type)]
    pub asset_type: AssetType,
    #[column_id(denom)]
    pub asset_denom: Denom,
    pub account_status: AccountStatus,
    pub account_type: AccountType,
    pub account_business_type: AccountBusinessType,
    pub account_role: AccountRole,
}

/*struct AccountRow {
    id: Id, 
    owner_id: Id, 
    asset_type SERIAL REFERENCES asset_type(id),
    account_type ACCOUNT_TYPE NOT NULL,
    account_business_type account_business_type NOT NULL,
    account_role account_role NOT NULL,
    account_status account_status_type NOT NULL DEFAULT 'active'
}*/

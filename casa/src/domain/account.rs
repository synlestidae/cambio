use db::{TryFromRow, TryFromRowError};
use domain::{
    AccountBusinessType, AccountRole, AccountStatus, AccountType, AssetType, Id, OwnerId,
};
use postgres;
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, TryFromRow, Serialize)]
pub struct Account {
    #[column_id(account_id)]
    pub id: Option<Id>,
    #[column_id(owner_id)]
    pub owner_user_id: Option<OwnerId>,
    #[column_id(account_asset_type)]
    pub asset_type: AssetType,
    pub account_status: AccountStatus,
    pub account_type: AccountType,
    pub account_business_type: AccountBusinessType,
    pub account_role: AccountRole,
}

impl Account {
    pub fn is_user_visible(&self) -> bool {
        return self.account_business_type == AccountBusinessType::UserCashWallet
            && self.account_role == AccountRole::Primary;
    }
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

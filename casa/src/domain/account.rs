use db::{TryFromRow, TryFromRowError};
use domain::{
    AccountBusinessType, AccountId, AccountRole, AccountStatus, AccountType, AssetType,
    CurrencyCode, OwnerId,
};
use postgres;
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, TryFromRow, Serialize)]
pub struct Account {
    #[column_id(account_id)]
    pub id: Option<AccountId>,
    #[column_id(owner_id)]
    pub owner_user_id: Option<OwnerId>,
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

    pub fn get_currency_code(&self) -> Option<CurrencyCode> {
        if self.asset_type == AssetType::NZD {
            Some(CurrencyCode::NZD)
        } else {
            None
        }
    }

    pub fn new_wallet(asset_type: AssetType) -> Self {
        Self {
            id: None,
            owner_user_id: None,
            asset_type: asset_type,
            account_status: AccountStatus::Active,
            account_type: AccountType::Liability,
            account_business_type: AccountBusinessType::UserCashWallet,
            account_role: AccountRole::Primary,
        }
    }

    pub fn new_hold(asset_type: AssetType) -> Self {
        Self {
            id: None,
            owner_user_id: None,
            asset_type: asset_type,
            account_status: AccountStatus::Active,
            account_type: AccountType::Liability,
            account_business_type: AccountBusinessType::OrderPaymentHold,
            account_role: AccountRole::System,
        }
    }

    pub fn is_for_deducting_payments(&self) -> bool {
        self.account_business_type == AccountBusinessType::SystemFeesPaid
            && self.account_role == AccountRole::System
    }

    pub fn is_for_wallet(&self) -> bool {
        self.account_business_type == AccountBusinessType::UserCashWallet
            && self.account_role == AccountRole::Primary
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

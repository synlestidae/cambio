use db;
use domain;

pub struct AccountSet {
    nzd_wallet_account: domain::Account,
    nzd_holding_account: domain::Account,
}

impl AccountSet {
    pub fn from(accounts: Vec<domain::Account>) -> Result<Self, db::CambioError> {
        let w = accounts
            .clone()
            .into_iter()
            .filter(|a| is_nzd_wallet(a))
            .collect::<Vec<_>>()
            .pop();

        let h = accounts
            .clone()
            .into_iter()
            .filter(|a| is_nzd_hold(a))
            .collect::<Vec<_>>()
            .pop();

        match (w, h) {
            (Some(wallet), Some(hold)) => Ok(AccountSet {
                nzd_wallet_account: wallet,
                nzd_holding_account: hold,
            }),
            otherwise => {
                return Err(db::CambioError::not_found_search(
                    "Could not find all accounts for user",
                    "User is missing wallet and/or hold account",
                ));
            }
        }
    }

    pub fn nzd_wallet(&self) -> domain::AccountId {
        self.nzd_wallet_account.id.unwrap() //TODO  Remove unwrap()
    }

    pub fn nzd_hold(&self) -> domain::AccountId {
        self.nzd_holding_account.id.unwrap() //TODO Remove unwrap()
    }

    pub fn nzd_pledge_hold(&self) -> domain::AccountId {
        self.nzd_holding_account.id.unwrap() //TODO Remove unwrap()
    }
}

fn is_nzd_wallet(a: &domain::Account) -> bool {
    a.asset_type == domain::AssetType::NZD
        && a.account_business_type == domain::AccountBusinessType::UserCashWallet
        && a.account_role == domain::AccountRole::Primary
}

fn is_nzd_hold(a: &domain::Account) -> bool {
    a.asset_type == domain::AssetType::NZD
        && a.account_business_type == domain::AccountBusinessType::OrderPaymentHold
        && a.account_role == domain::AccountRole::System
}

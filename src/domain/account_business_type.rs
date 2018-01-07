use std::fmt;
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountBusinessType {
    UserCashWallet,
    UserCashoutCredit,
    SystemFeesPaid,
    UserGenericAsset,
    AccountingConcept
}

impl AccountBusinessType {
    pub fn parse(account_business_type: &str) -> Option<AccountBusinessType> {
        match account_business_type {
            "user_cash_wallet" => Some(AccountBusinessType::UserCashWallet),
            "user_cashout_credit" => Some(AccountBusinessType::UserCashoutCredit),
            "system_fees_paid" => Some(AccountBusinessType::SystemFeesPaid),
            "user_generic_asset" => Some(AccountBusinessType::UserGenericAsset),
            "accounting_concept" => Some(AccountBusinessType::AccountingConcept),
            _ => None
        }
    }
}

impl fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
             &AccountBusinessType::UserCashWallet => "user_cash_wallet",
             &AccountBusinessType::UserCashoutCredit => "user_cashout_credit",
             &AccountBusinessType::SystemFeesPaid => "system_fees_paid",
             &AccountBusinessType::UserGenericAsset => "user_generic_asset",
             &AccountBusinessType::AccountingConcept => "accounting_concept",
        };
        write!(f, "{}", string)
    }
}

impl TryFromRow for AccountBusinessType {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let account_business_type_match: Option<String> = row.get("account_business_type");
        let account_business_type = try!(account_business_type_match.ok_or(TryFromRowError{}));

        match AccountBusinessType::parse(&account_business_type) {
            Some(b) => Ok(b),
            _ => Err(TryFromRowError {})
        }
    }
}

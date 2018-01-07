use std::fmt;
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "account_business_type")]
pub enum AccountBusinessType {
    #[postgres(name = "user_cash_wallet")]
    UserCashWallet,
    #[postgres(name = "system_fees_paid")]
    SystemFeesPaid,
    #[postgres(name = "accounting_concept")]
    AccountingConcept
}

impl AccountBusinessType {
    pub fn parse(account_business_type: &str) -> Option<AccountBusinessType> {
        match account_business_type {
            "user_cash_wallet" => Some(AccountBusinessType::UserCashWallet),
            "system_fees_paid" => Some(AccountBusinessType::SystemFeesPaid),
            "accounting_concept" => Some(AccountBusinessType::AccountingConcept),
            _ => None
        }
    }
}

impl fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
             &AccountBusinessType::UserCashWallet => "user_cash_wallet",
             &AccountBusinessType::SystemFeesPaid => "system_fees_paid",
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

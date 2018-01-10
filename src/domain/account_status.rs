use std::fmt;
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "account_status_type")]
pub enum AccountStatus {
    #[postgres(name = "active")]
    Active,
    #[postgres(name = "frozen")]
    Frozen,
    #[postgres(name = "closed")]
    Closed,
}

impl AccountStatus {
    pub fn parse(business_ends: &str) -> Option<AccountStatus> {
        match business_ends {
            "active" => Some(AccountStatus::Active),
            "frozen" => Some(AccountStatus::Frozen),
            "closed" => Some(AccountStatus::Closed),
            _ => None,
        }
    }
}

impl TryFromRow for AccountStatus {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let account_status_match: Option<String> = row.get("account_status");
        let account_status = try!(account_status_match.ok_or(TryFromRowError {}));

        match account_status.as_ref() {
            "active" => Ok(AccountStatus::Active),
            "frozen" => Ok(AccountStatus::Frozen),
            "closed" => Ok(AccountStatus::Closed),
            _ => Err(TryFromRowError {}),
        }
    }
}

impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &AccountStatus::Active => "active",
            &AccountStatus::Frozen => "frozen",
            &AccountStatus::Closed => "closed",
        };
        write!(f, "{}", string)
    }
}

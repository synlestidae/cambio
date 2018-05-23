use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize)]
#[postgres(name = "account_status_type")]
pub enum AccountStatus {
    #[postgres(name = "active")]
    Active,
    #[postgres(name = "frozen")]
    Frozen,
    #[postgres(name = "closed")]
    Closed,
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

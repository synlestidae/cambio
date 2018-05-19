use db::{TryFromRow, TryFromRowError};
use std::fmt;
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "account_role")]
pub enum AccountRole {
    #[postgres(name = "primary")] Primary,
    #[postgres(name = "system")] System,
}

impl fmt::Display for AccountRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &AccountRole::Primary => "primary",
            &AccountRole::System => "system",
        };
        write!(f, "{}", string)
    }
}

use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "account_role")]
pub enum AccountRole {
    #[postgres(name = "primary")]
    Primary,
    #[postgres(name = "system")]
    System,
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

impl AccountRole {
    pub fn is_for_wallet(&self) -> bool {
        self == &AccountRole::Primary
    }

    pub fn is_for_deducting_payments(&self) -> bool {
        self == &AccountRole::Primary
    }
}

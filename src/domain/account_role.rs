use db::{TryFromRow, TryFromRowError};
use std::fmt;
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountRole {
    Primary,
    System
}

impl fmt::Display for AccountRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
             &AccountRole::Primary => "primary",
             &AccountRole::System=> "system"
        };
        write!(f, "{}", string)
    }
}

impl TryFromRow for AccountRole {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let account_role_match: Option<String> = row.get("account_role");
        let account_role = try!(account_role_match.ok_or(TryFromRowError{}));

        match account_role.as_ref() {
            "primary" => Ok(AccountRole::Primary),
            "system" => Ok(AccountRole::System),
            _ => Err(TryFromRowError {})
        }
    }
}

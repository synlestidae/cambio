use db::CambioError;
use postgres;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use std;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Write;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, SqlId)]
pub struct Id(pub i32);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

fn err() -> CambioError {
    CambioError::shouldnt_happen(
        "Couldn't convert database data",
        "Error converting Id to or from INT4",
    )
}

fn accepts(ty: &Type) -> bool {
    true
}

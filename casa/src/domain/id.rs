use db::CambioError;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use postgres;
use std::fmt::Display;
use std::io::Cursor;
use std::io::Write;
use std;

type ToSqlResult = Result<IsNull, Box<std::error::Error + 'static + Send + Sync>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id(pub i32);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl ToSql for Id {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.0.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        accepts(ty)
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.0.to_sql_checked(ty, out)
    }
}

impl FromSql for Id {
    fn from_sql(
        ty: &Type,
        raw: &[u8],
    ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
        let id = try!(i32::from_sql(ty, raw));
        Ok(Id(id))
    }

    fn accepts(ty: &Type) -> bool {
        accepts(ty)
    }

    fn from_sql_null(ty: &Type) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
        let id = try!(i32::from_sql_null(ty));
        Ok(Id(id))
    }

    fn from_sql_nullable(
        ty: &Type,
        raw: Option<&[u8]>,
    ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
        let id = try!(i32::from_sql_nullable(ty, raw));
        Ok(Id(id))
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

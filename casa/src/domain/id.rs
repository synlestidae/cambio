use postgres;
use postgres::types::{ToSql, FromSql, Type};
use std::io::Write;
use postgres::types::IsNull;
//use postgres::error::Error;
use db::CambioError;
use byteorder::{ByteOrder, LittleEndian};
use std;
use std::fmt::Display;
use std::io::Cursor;

type ToSqlResult = Result<IsNull, Box<std::error::Error + 'static + Send + Sync>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Id(pub i32);

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self)
    }
}

impl ToSql for Id {
    fn to_sql(
        &self, 
        ty: &Type, 
        out: &mut Vec<u8>) -> ToSqlResult {
        let mut buf = Vec::new();
        LittleEndian::write_i32(&mut buf, self.0);
        Ok(IsNull::No)
    }
    fn accepts(ty: &Type) -> bool where Self: Sized {
        true
        //ty ==  Type::Int4
        // fix this shit!
    }
    fn to_sql_checked(&self, 
        ty: &Type, 
        out: &mut Vec<u8>) -> ToSqlResult {
        let accepts: bool = Id::accepts(ty);
        if accepts {
            self.to_sql(ty, out)
        } else {
            Err(Box::new(err()))
        }
    }

}

impl FromSql for Id {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> {
        unimplemented!()
    }
    fn accepts(ty: &Type) -> bool {
        unimplemented!()
    }

    fn from_sql_null(
        ty: &Type
    ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> { 
        Err(Box::new(CambioError::shouldnt_happen("Error converting a database value", "Cannot convert NULL to i32")))
    }
    fn from_sql_nullable(
        ty: &Type, 
        raw: Option<&[u8]>
    ) -> Result<Self, Box<std::error::Error + 'static + Send + Sync>> { 
        match raw {
            None => FromSql::from_sql_null(ty),
            Some(bytes) => {
                FromSql::from_sql(ty, bytes)
            }
        }
    }
}

fn err() -> CambioError {
    CambioError::shouldnt_happen("Error converting data", "Attempted to convert Id to non-INT4")
}

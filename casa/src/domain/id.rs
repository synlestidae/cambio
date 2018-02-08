use postgres;
use postgres::types::{ToSql, Type};
use std::io::Write;
use postgres::types::IsNull;
//use postgres::error::Error;
use db::CambioError;
use byteorder::{ByteOrder, LittleEndian};
use std;

type ToSqlResult = Result<IsNull, Box<std::error::Error + 'static + Send + Sync>>;

#[derive(Debug, Clone, Copy)]
pub struct Id(pub i32);

impl ToSql for Id {
    fn to_sql(
        &self, 
        ty: &Type, 
        out: &mut Vec<u8>) -> ToSqlResult {
        let mut buf = Vec::new();
        LittleEndian::write_u32(&mut buf, self);
        Ok(IsNull::No)
    }
    fn accepts(ty: &Type) -> bool where Self: Sized {
        ty ==  Type::Int4
    }
    fn to_sql_checked(&self, 
        ty: &Type, 
        out: &mut Vec<u8>) -> ToSqlResult {
        if ToSql::accepts(self, ty) {
            self.to_sql(ty, out)
        } else {
            Err(postgres::Error::Conversion(Box::new(err())))
        }
    }

}

fn err() -> CambioError {
    CambioError::shouldnt_happen("Error converting data", "Attempted to convert Id to non-INT4")
}

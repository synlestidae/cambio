use postgres::types::{FromSql, IsNull, ToSql, Type};
use std::error::Error;

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct IdentifierCode(pub String);

impl FromSql for IdentifierCode {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        match String::from_utf8(raw.iter().map(|&x| x).collect()) {
            Ok(s) => Ok(IdentifierCode(s)),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl ToSql for IdentifierCode {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut Vec<u8>,
    ) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
        out.append(&mut bytes);
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut Vec<u8>,
    ) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
        out.append(&mut bytes);
        Ok(IsNull::No)
    }
}

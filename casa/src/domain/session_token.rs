use postgres::types::{FromSql, IsNull, ToSql, Type};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SessionToken(pub String);

impl fmt::Display for SessionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql for SessionToken {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        match String::from_utf8(raw.iter().map(|&x| x).collect()) {
            Ok(s) => Ok(SessionToken(s)),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl ToSql for SessionToken {
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

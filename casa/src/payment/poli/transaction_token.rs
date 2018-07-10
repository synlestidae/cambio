use postgres::types::{ToSql, FromSql, Type, IsNull};
use serde::{Deserialize, Deserializer};
use std::error::Error;

#[derive(Eq, PartialEq, Clone, Serialize, Debug)]
pub struct TransactionToken(String);

impl FromSql for TransactionToken {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        match String::from_utf8(raw.iter().map(|&x| x).collect()) {
            Ok(s) => Ok(TransactionToken(s)),
            Err(err) => Err(Box::new(err))
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl ToSql for TransactionToken {
	fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
		out.append(&mut bytes);
		Ok(IsNull::No)
	}

	fn accepts(ty: &Type) -> bool {
		true
	}

	fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) 
		-> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
		out.append(&mut bytes);
        Ok(IsNull::No)
	}
}

impl<'de> Deserialize<'de> for TransactionToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = try!(String::deserialize(deserializer));
        Ok(TransactionToken(data))
    }
}

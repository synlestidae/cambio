use postgres::types::{ToSql, FromSql, Type, IsNull};
use std::error::Error;

#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub struct TransactionRefNo(String);

impl FromSql for TransactionRefNo {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        match String::from_utf8(raw.iter().map(|&x| x).collect()) {
            Ok(s) => Ok(TransactionRefNo(s)),
            Err(err) => Err(Box::new(err))
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl ToSql for TransactionRefNo {
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

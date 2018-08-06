use web3::types::H160;
use postgres::types::*;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct ByteAddress(H160); 

impl From<H160> for ByteAddress {
    fn from(h: H160) -> Self {
        ByteAddress(h)
    }
}

impl Into<H160> for ByteAddress {
    fn into(self) -> H160 {
        self.0
    }
}

impl FromSql for ByteAddress {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
		Ok(to_H160(&Vec::from_sql(ty, raw)?))
	}

    fn accepts(ty: &Type) -> bool {
		ty.name().to_ascii_lowercase() == "bytea"	
	}

    fn from_sql_null(ty: &Type) -> Result<Self, Box<Error + 'static + Send + Sync>> { 
		Ok(to_H160(&Vec::from_sql_null(ty)?))
        
	}

    fn from_sql_nullable(ty: &Type, raw: Option<&[u8]>) -> Result<Self, Box<Error + 'static + Send + Sync>> { 
		Ok(to_H160(&Vec::from_sql_nullable(ty, raw)?))

	}
}


impl ToSql for ByteAddress {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut sql_array = Vec::new();
        sql_array.resize(20usize, 0u8);
        self.0.copy_to(&mut sql_array);
        let val = sql_array.to_sql(ty, out);
        val
    }

    fn accepts(ty: &Type) -> bool {
		ty.name().to_ascii_lowercase() == "bytea"	
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut sql_array = Vec::new();
        sql_array.resize(20usize, 0u8);
        self.0.copy_to(&mut sql_array);
        sql_array.to_sql_checked(ty, out)
    }
}

fn to_H160(bytes: &[u8]) -> ByteAddress {
    use std::cmp::min;
	let mut result: [u8; 20] = [0; 20];
	for i in 0..min(bytes.len(), 20) {
        result[i] = bytes[i]; 	
    }
    ByteAddress(result.into())
}

use web3::types::U256;
//use serde::{Deserialize, Deserializer};
use std::ops::*;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use std::error;
use byteorder;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct BigInteger(U256);

/*impl<'de> Deserialize<'de> for BigInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = U256::deserialize(deserializer)?;
        Ok(BigInteger(data))
    }
}*/

impl Add for BigInteger {
    type Output = Self;

    fn add(self, other: BigInteger) -> BigInteger {
        BigInteger(self.0 + other.0)
    }
}

impl Sub for BigInteger {
    type Output = Self;

    fn sub(self, other: BigInteger) -> BigInteger {
        BigInteger(self.0 - other.0)
    }
}

type ToSqlResult = Result<IsNull, Box<error::Error + 'static + Send + Sync>>;

impl ToSql for BigInteger {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        let mut buf = Vec::new();
        buf.resize(32usize, 0u8);
        self.0.to_little_endian(&mut buf);
        buf.to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        let mut buf = Vec::new();
        buf.resize(32usize, 0u8);
        self.0.to_little_endian(&mut buf);
        buf.to_sql_checked(ty, out)
    }
}

impl FromSql for BigInteger {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let buf = Vec::from_sql(ty, raw)?;
        Ok(BigInteger(U256::from_little_endian(&buf)))
        
    }

    fn accepts(ty: &Type) -> bool {
        true
    }

    fn from_sql_null(ty: &Type) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let buf = Vec::from_sql_null(ty)?;
        Ok(BigInteger(U256::from_little_endian(&buf)))
    }

    fn from_sql_nullable(
        ty: &Type,
        raw: Option<&[u8]>,
    ) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let buf = Vec::from_sql_nullable(ty, raw)?;
        Ok(BigInteger(U256::from_little_endian(&buf)))
    }
}

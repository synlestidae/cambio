use web3::types::U256;
//use serde::{Deserialize, Deserializer};
use std::ops::*;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use std::error;
use byteorder;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Copy, Debug)]
pub struct BigInteger(U256);

impl BigInteger {
}

impl Into<U256> for BigInteger {
    fn into(self) -> U256 {
        self.0.clone()
    }
}

impl From<U256> for BigInteger {
    fn from(u: U256) -> Self {
        BigInteger(u)
    }
}

impl From<u64> for BigInteger {
    fn from(u: u64) -> Self {
        BigInteger(U256::from(u))
    }
}

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

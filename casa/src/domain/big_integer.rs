use web3::types::U256;
use serde::{Deserialize, Deserializer};
use std::ops::*;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};

#[derive(Clone, Copy)]
pub struct BigInteger(U256);

impl<'de> Deserialize<'de> for BigInteger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = U256::deserialize(deserializer)?;
        Self(data)
    }
}

impl Add for BigInteger {
    type Output = Self

    fn add(self, other: BigInteger) -> BigInteger {
        Self(self.0 + other.0)
    }
}

impl Sub for BigInteger {
    type Output = Self

    fn sub(self, other: BigInteger) -> BigInteger {
        Self(self.0 - other.0)
    }
}

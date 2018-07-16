use std::ops::{Add, Sub};
use db::CambioError;
use std::str::FromStr;
use std::fmt;
use serde::*;
use std;
use postgres::types::{FromSql, ToSql, Type};
use postgres::types::IsNull;
use std::error::Error;

type ToSqlResult = Result<IsNull, Box<Error + 'static + Send + Sync>>;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Decimal {
    is_positive: bool,
    dollars: u64,
    cents: u64
}

impl Decimal {
    pub fn new() -> Self {
        Self::from_dollars(0)
    }
    pub fn from_dollars(dollars: i64) -> Self {
        Self {
            dollars: dollars.abs() as u64,
            is_positive: dollars >= 0,
            cents: 0
        }
    }

    pub fn from_cents(cents: i64) -> Self {
        let cents_100 = cents.abs() % 100;
        let dollars = cents.abs() / 100;
        Self {
            is_positive: cents >= 0,
            dollars: dollars as u64,
            cents: cents_100 as u64
        }
    }

    pub fn to_cents(&self) -> i64 {
        let d = self.dollars as i64;
        let c = self.cents as i64;
        (d * 100) + c * if self.is_positive { 1 } else { -1 } 
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = self.to_cents() + other.to_cents();
        Self::from_cents(sum)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let difference = self.to_cents() - other.to_cents();
        Self::from_cents(difference)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.is_positive {
            ""
        } else {
            "-"
        };
        write!(f, "{}{}.{:02}", sign, self.dollars, self.cents)
    }
}

impl FromStr for Decimal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = match s.to_owned().chars().next() {
            Some('-') => -1,
            _ => 1
        };
        match s.split(".").map(|s| s.to_owned()).collect::<Vec<String>>().as_slice() {
            &[ref dollars, ref cents] => {
                match (i64::from_str(dollars), u64::from_str(cents)) {
                    (Ok(d), Ok(c)) => {
                        if cents.len() != 2 {
                            Err(())
                        } else {
                            Ok(Self::from_cents(sign * ((d.abs() * 100) + c as i64)))
                        }
                    },
                    _ => Err(())
                }
            },
            _ => Err(())
        }
    }
}

impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        // TODO Unwrap is unnacceptable!
        let currency_val = String::deserialize(deserializer).unwrap();
        Ok(Self::from_str(&currency_val).unwrap())
    }
}

impl ToSql for Decimal {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.cents.to_string().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.to_string().to_sql_checked(ty, out)
    }
}

impl FromSql for Decimal {
    fn from_sql(
        ty: &Type,
        raw: &[u8],
    ) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        let value = try!(i64::from_sql(ty, raw));
        Ok(Decimal::from_cents(value))//.map_err(|_| err())
    }

    fn accepts(ty: &Type) -> bool {
        true
    }

    fn from_sql_null(ty: &Type) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        let value = try!(String::from_sql_null(ty));
        Decimal::from_str(&value).map_err(|_| err())
    }

    fn from_sql_nullable(
        ty: &Type,
        raw: Option<&[u8]>,
    ) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        let value = try!(String::from_sql_nullable(ty, raw));
        Decimal::from_str(&value).map_err(|_| err())
    }
}

fn err() -> Box<Error + Sync + Send + 'static> {
    unimplemented!()
}

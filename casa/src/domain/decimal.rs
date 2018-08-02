use db::CambioError;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use serde::de::Error;
use serde::*;
use serde_json::Value;
use std;
use std::error; //::Error as StdError;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::convert::Into;

type ToSqlResult = Result<IsNull, Box<error::Error + 'static + Send + Sync>>;

#[derive(Eq, PartialEq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Decimal {
    cents: i64,
}

impl Decimal {
    pub fn new() -> Self {
        Self::from_dollars(0)
    }
    pub fn from_dollars(dollars: i64) -> Self {
        Self {
            cents: dollars * 100,
        }
    }

    pub fn from_cents(cents: i64) -> Self {
        Self {
            cents: cents
        }
    }

    pub fn to_cents(&self) -> i64 {
        self.cents
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
        write!(f, "{}", self.cents)
    }
}

impl FromStr for Decimal {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sign = match s.to_owned().chars().next() {
            Some('-') => -1,
            _ => 1,
        };
        match s
            .split(".")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .as_slice()
        {
            &[ref dollars, ref cents] => match (i64::from_str(dollars), u64::from_str(cents)) {
                (Ok(d), Ok(c)) => {
                    if cents.len() > 0 && cents.len() <= 2 {
                        Ok(Self::from_cents(sign * ((d.abs() * 100) + c as i64)))
                    } else {
                        Err("Decimal should have one or two decimal places")
                    }
                }
                _ => Err("Could not parse the two figures"),
            },
            &[ref dollars] => match i64::from_str(dollars) {
                Ok(d) => Ok(Self::from_cents(sign * ((d.abs() * 100)))),
                _ => Err("Dollar amount without decimal places is not valid"),
            },
            _ => Err("Figure appears to have multiple decimal places"),
        }
    }
}

impl Serialize for Decimal {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Decimal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO Unwrap is unnacceptable!
        let currency_val = Value::deserialize(deserializer)?;
        let decimal_string: String = if let Value::Number(num) = currency_val {
            num.to_string()
        } else if let Value::String(s) = currency_val {
            s.to_string()
        } else {
            return Err(D::Error::custom(format!(
                "Can only deserialize Decimal from string or number"
            )));
        };
        match Self::from_str(&decimal_string) {
            Err(err) => Err(D::Error::custom(err)),
            Ok(val) => Ok(val),
        }
    }
}

impl ToSql for Decimal {
    fn to_sql(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.to_cents().to_sql(ty, out)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        true
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut Vec<u8>) -> ToSqlResult {
        self.to_cents().to_sql_checked(ty, out)
    }
}

impl FromSql for Decimal {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let numeric_val = i64::from_sql(ty, raw)?;
        Ok(Self::from_cents(numeric_val))
    }

    fn accepts(ty: &Type) -> bool {
        true
    }

    fn from_sql_null(ty: &Type) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let numeric_val = i64::from_sql_null(ty)?;
        Ok(Self::from_cents(numeric_val))
    }

    fn from_sql_nullable(
        ty: &Type,
        raw: Option<&[u8]>,
    ) -> Result<Self, Box<error::Error + 'static + Send + Sync>> {
        let numeric_val = i64::from_sql_nullable(ty, raw)?;
        Ok(Self::from_cents(numeric_val))
    }
}

fn err_currency_format(e: &str) -> Box<error::Error + Send + Sync + 'static> {
    Box::new(CambioError::format_obj(
        "Currency was in incorrect format",
        e,
    ))
}

fn err_format_obj(e: &str) -> Box<error::Error + Send + Sync + 'static> {
    Box::new(CambioError::format_obj(
        "Failed to load currency from database.",
        e,
    ))
}

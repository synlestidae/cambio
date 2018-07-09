use std::ops::{Add, Sub};
use std::str::FromStr;
use std::fmt;
use serde::*;

#[derive(Eq, PartialEq, Debug)]
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

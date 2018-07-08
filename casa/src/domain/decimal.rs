use std::ops::{Add, Sub};
use std::str::FromStr;
use std::fmt;
use serde::*;

#[derive(Eq, PartialEq)]
pub struct Decimal {
    dollars: u64,
    cents: u64
}

impl Decimal {
    pub fn new() -> Self {
        Self::from_dollars(0)
    }
    pub fn from_dollars(dollars: u64) -> Self {
        Self {
            dollars: dollars,
            cents: 0
        }
    }

    pub fn from_cents(cents: u64) -> Self {
        let cents_100 = cents % 100;
        let dollars = cents / 100;
        Self {
            dollars: dollars,
            cents: cents_100 
        }
    }

    pub fn to_cents(&self) -> u64 {
        self.dollars * 100 + self.cents
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let sum = (self.dollars * 100) + self.cents + (other.dollars * 100) + other.cents;
        Self::from_cents(sum)
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let difference = (other.dollars * 100) + other.cents + (self.dollars * 100) + self.cents;
        Self::from_cents(difference)
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.dollars, self.cents)
    }
}

impl FromStr for Decimal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(".").map(|s| s.to_owned()).collect::<Vec<String>>().as_slice() {
            &[ref dollars, ref cents] => {
                match (u64::from_str(dollars), u64::from_str(cents)) {
                    (Ok(d), Ok(c)) => {
                        if cents.len() != 2 {
                            Err(())
                        } else {
                            Ok(Self::from_cents((d * 100) + c))
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

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum CurrencyCode {
    NZD,
    AUD
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

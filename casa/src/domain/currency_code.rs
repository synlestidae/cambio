use domain::AssetType;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum CurrencyCode {
    NZD,
    AUD,
}

impl CurrencyCode {
    pub fn asset_type(&self) -> AssetType {
        match self {
            CurrencyCode::NZD => AssetType::NZD,
            CurrencyCode::AUD => AssetType::AUD,
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

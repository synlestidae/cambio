use std::fmt;
use domain::AssetType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum CurrencyCode {
    NZD,
    AUD,
}

impl CurrencyCode {
    pub fn asset_type(&self) -> AssetType {
        match self {
            CurrencyCode::NZD => AssetType::NZD,
            _ => unimplemented!()
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

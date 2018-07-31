use domain::AssetType;
use std::fmt;

#[derive(Serialize, Deserialize, ToSql, FromSql, Debug, Eq, PartialEq, Clone)]
#[postgres(name = "currency_code")]
pub enum CurrencyCode {
    #[postgres(name = "nzd")]
    NZD,
    #[postgres(name = "aud")]
    AUD
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

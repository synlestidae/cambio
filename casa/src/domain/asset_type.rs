use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "asset_code_type")]
pub enum AssetType {
    #[postgres(name = "nzd")] NZD,
    #[postgres(name = "btc")] BTC,
    #[postgres(name = "eth")] ETH,
}

impl AssetType {
    pub fn is_crypto(&self) -> bool {
        match self {
            &AssetType::BTC => true,
            &AssetType::ETH => true,
            _ => false,
        }
    }
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        let asset_type_str = match self {
            &AssetType::NZD => "nzd",
            &AssetType::BTC => "btc",
            &AssetType::ETH => "eth",
        };
        asset_type_str.to_owned()
    }
}

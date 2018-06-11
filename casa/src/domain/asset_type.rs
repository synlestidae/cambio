use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "asset_type")]
pub enum AssetType {
    #[postgres(name = "nzd_cent")]
    NZD,
    #[postgres(name = "eth_wei")]
    ETH,
}

impl AssetType {
    pub fn is_crypto(&self) -> bool {
        match self {
            &AssetType::ETH => true,
            _ => false,
        }
    }
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        let asset_type_str = match self {
            &AssetType::NZD => "nzd",
            &AssetType::ETH => "eth"
        };
        asset_type_str.to_owned()
    }
}

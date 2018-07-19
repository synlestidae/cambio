use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "asset_type")]
pub enum AssetType {
    #[postgres(name = "nzd_cent")]
    NZD,
    #[postgres(name = "eth_wei")]
    ETH,
    #[postgres(name = "eth_szabo")]
    ETHSzabo,
}

impl AssetType {
    pub fn is_crypto(&self) -> bool {
        match self {
            &AssetType::ETH => true,
            &AssetType::ETHSzabo => true,
            _ => false,
        }
    }
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        let asset_type_str = match self {
            &AssetType::NZD => "nzd",
            &AssetType::ETH => "eth_wei",
            &AssetType::ETHSzabo => "eth_szabo",
        };
        asset_type_str.to_owned()
    }
}

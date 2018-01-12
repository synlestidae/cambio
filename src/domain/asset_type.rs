use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum AssetType {
    NZD,
    BTC,
    ETH,
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

impl TryFromRow for AssetType {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let asset_type_match: Option<String> = row.get("asset_code");
        if asset_type_match.is_none() {
            return Err(TryFromRowError::missing_field("AssetType", "asset_code"));
        }
        match asset_type_match.unwrap().as_ref() {
            "eth" => Ok(AssetType::ETH),
            "btc" => Ok(AssetType::BTC),
            "nzd" => Ok(AssetType::NZD),
            unknown => Err(TryFromRowError::unknown_value("AssetType", unknown)),
        }
    }
}

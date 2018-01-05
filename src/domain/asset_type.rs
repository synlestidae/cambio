use serde_derive;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    NZD,
    BTC,
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        let asset_type_str = match self {
            &AssetType::NZD => "nzd",
            &AssetType::BTC => "btc",
        };
        asset_type_str.to_owned()
    }
}

impl TryFromRow for AssetType {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        for c in row.columns() {}
        unimplemented!();
    }
}

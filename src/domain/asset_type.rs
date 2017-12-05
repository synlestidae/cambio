use serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AssetType {
    NZD,
    BTC
}

impl ToString for AssetType {
    fn to_string(&self) -> String {
        let asset_type_str = match self {
            &AssetType::NZD => "nzd",
            &AssetType::BTC => "btc"
        };
        asset_type_str.to_owned()
    }
}

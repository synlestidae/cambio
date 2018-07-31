#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "asset_type")]
pub enum TradeType {
    #[postgres(name = "buy_crypto")]
    BuyCrypto,
    #[postgres(name = "sell_crypto")]
    SellCrypto
}

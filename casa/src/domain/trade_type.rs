#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "trade_type")]
pub enum TradeType {
    #[postgres(name = "buy_crypto")]
    BuyCrypto,
    #[postgres(name = "sell_crypto")]
    SellCrypto
}

impl TradeType {
    pub fn is_compatible(&self, other: &TradeType) -> bool {
        return self != other;
    }
}

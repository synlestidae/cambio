#[derive(Debug, Clone, Copy, Eq, PartialEq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "crypto_type")]
pub enum CryptoType {
    #[postgres(name = "ether")]
    Ether
}

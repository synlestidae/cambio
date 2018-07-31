#[postgres(name = "business_ends_type")]
#[derive(Debug, Clone, Copy, Eq, PartialEq, ToSql, FromSql, Serialize, Deserialize)]
pub enum CryptoType {
    #[postgres(name = "ether")]
    Ether
}

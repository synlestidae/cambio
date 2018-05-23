use domain::{Id};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, ToSql, FromSql)]
pub struct OrderSettlementId(pub i32);

impl Into<Id> for OrderSettlementId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

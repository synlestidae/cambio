use domain::{Id};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, ToSql, FromSql)]
pub struct OrderId(pub i32);

impl Into<Id> for OrderId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

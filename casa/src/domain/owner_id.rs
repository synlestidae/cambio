use domain::Id;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, ToSql,
         FromSql)]
pub struct OwnerId(pub i32);

impl Into<Id> for OwnerId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

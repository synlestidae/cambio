use domain::Id;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, ToSql,
         FromSql)]
pub struct UserId(pub i32);

impl Into<Id> for UserId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

use domain::Id;
use postgres::types::{FromSql, ToSql, Type};
use postgres::types::IsNull;
use std;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, SqlId)]
pub struct EthAccountId(pub i32);

impl Into<Id> for EthAccountId {
    fn into(self) -> Id {
        Id(self.0)
    }
}
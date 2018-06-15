use domain::Id;
use postgres::types::{FromSql, ToSql, Type};
use postgres::types::IsNull;
use std;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, SqlId)]
pub struct TransactionId(pub i32);

impl Into<Id> for TransactionId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

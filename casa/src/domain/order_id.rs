use domain::Id;
use postgres::types::{FromSql, ToSql, Type};
use postgres::types::IsNull;
use std;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, SqlId)]
pub struct OrderId(pub i32);

impl Into<Id> for OrderId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

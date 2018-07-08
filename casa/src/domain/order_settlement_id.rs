use domain::Id;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use std;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, SqlId)]
pub struct OrderSettlementId(pub i32);

impl Into<Id> for OrderSettlementId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

use domain::Id;
use postgres::types::IsNull;
use postgres::types::{FromSql, ToSql, Type};
use std;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, SqlId)]
pub struct ProfileId(pub i32);

impl Into<Id> for ProfileId {
    fn into(self) -> Id {
        Id(self.0)
    }
}

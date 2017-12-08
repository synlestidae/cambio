use std;
use postgres::rows::Row;

pub trait TryFromRow {
    type Error;
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, Self::Error> where Self: std::marker::Sized;
}

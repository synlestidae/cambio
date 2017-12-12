use std;
use postgres::rows::Row;
use db::try_from_row_error::TryFromRowError;

pub trait TryFromRow {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> where Self: std::marker::Sized;
}

use repository::column_name::ColumnName;
use postgres::types::ToSql;
use std;

pub trait Clause {
    fn get_clause<'a>(&'a self) -> Vec<(ColumnName, String)>;
}

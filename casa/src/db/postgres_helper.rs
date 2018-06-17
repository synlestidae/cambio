use db::try_from_row::TryFromRow;
use db::CambioError;
use db::TransactionSource;
use db::Transaction;
use postgres::rows::Rows;
use postgres::types::ToSql;
use std::convert::From;
use std::marker::{Sync};
use db::PostgresPooledConn;

pub trait PostgresHelper {
    fn query<T: TryFromRow>(
        &mut self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, CambioError>;
    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, CambioError>;
    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, CambioError>;
}



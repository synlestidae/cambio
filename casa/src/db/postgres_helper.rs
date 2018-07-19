use db::try_from_row::TryFromRow;
use db::CambioError;
use db::PostgresPooledConn;
use db::Transaction;
use db::TransactionSource;
use postgres::rows::Rows;
use postgres::types::ToSql;
use postgres::GenericConnection;
use std::convert::From;
use std::marker::Sync;

pub trait PostgresHelper {
    fn query<T: TryFromRow, E: GenericConnection>(
        db: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, CambioError>;
    fn execute<E: GenericConnection>(
        db: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<u64, CambioError>;
    fn query_raw<E: GenericConnection>(
        db: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Rows, CambioError>;
}

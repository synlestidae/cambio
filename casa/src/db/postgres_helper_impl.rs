use db::try_from_row::TryFromRow;
use db::{
    CambioError, ConnectionSource, PostgresHelper, PostgresSource, Transaction, TransactionSource,
};
use postgres;
use postgres::rows::Rows;
use postgres::types::ToSql;
use postgres::GenericConnection;
use std::convert::From;
use std::error;
use std::error::Error;
use std::marker::{Send, Sync};

#[derive(Clone)]
pub struct PostgresHelperImpl;

unsafe impl Send for PostgresHelperImpl {}
unsafe impl Sync for PostgresHelperImpl {}

impl PostgresHelper for PostgresHelperImpl {
    fn query<T: TryFromRow, E: GenericConnection>(
        connection: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, CambioError> {
        let rows = try!(connection.query(query, params));
        let mut result_objs = Vec::new();
        for row in rows.iter() {
            let obj = try!(T::try_from_row(&row));
            result_objs.push(obj);
        }
        Ok(result_objs)
    }

    fn query_raw<E: GenericConnection>(
        conn: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Rows, CambioError> {
        let result = try!(conn.query(query, params));
        Ok(result)
    }

    fn execute<E: GenericConnection>(
        conn: &mut E,
        query: &str,
        params: &[&ToSql],
    ) -> Result<u64, CambioError> {
        let result = try!(conn.execute(query, params));
        Ok(result)
    }
}

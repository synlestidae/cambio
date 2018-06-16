use db::try_from_row::TryFromRow;
use db::{CambioError, ConnectionSource, PostgresHelper, PostgresSource, PostgresTransactionHelper, Transaction, TransactionSource};
use postgres;
use postgres::rows::Rows;
use postgres::types::ToSql;
use postgres::Connection;
use std::convert::From;
use std::error;
use std::error::Error;
use std::fmt;
use std::marker::{Send, Sync};
use web3;

#[derive(Clone)]
pub struct PostgresHelperImpl {
    conn_source: PostgresSource,
}

unsafe impl Send for PostgresHelperImpl {}
unsafe impl Sync for PostgresHelperImpl {}

impl PostgresHelperImpl {
    pub fn new(conn_source: PostgresSource) -> Self {
        PostgresHelperImpl {
            conn_source: conn_source,
        }
    }

    pub fn new_from_conn_str(conn_str: &str) -> Self {
        let source = PostgresSource::new(conn_str).unwrap();
        PostgresHelperImpl::new(source)
    }
}

impl PostgresHelper for PostgresHelperImpl {
    fn query<T: TryFromRow>(
        &mut self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, CambioError> {
        let connection = try!(self.conn_source.get());
        let rows = try!(connection.query(query, params));

        let mut result_objs = Vec::new();
        for row in rows.iter() {
            let obj = try!(T::try_from_row(&row));
            result_objs.push(obj);
        }
        Ok(result_objs)
    }

    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, CambioError> {
        let conn = try!(self.conn_source.get());
        let result = try!(conn.query(query, params));
        Ok(result)
    }

    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, CambioError> {
        let conn = try!(self.conn_source.get());
        let result = try!(conn.execute(query, params));
        Ok(result)
    }
}

impl<'a> TransactionSource<'a, PostgresTransactionHelper<'a>> for PostgresHelperImpl {
    fn begin_transaction(&'a mut self) -> Result<PostgresTransactionHelper<'a>, CambioError> {
        let connection = try!(self.conn_source.get());
        let tx = try!(connection.transaction());
        Ok(PostgresTransactionHelper::new(tx))
    }
}
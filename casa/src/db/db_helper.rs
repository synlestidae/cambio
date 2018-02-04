use postgres::rows::Rows;
use postgres::Connection;
use postgres::types::ToSql;
use postgres;
use db::try_from_row::TryFromRow;
use db::user_repository::UserRepository;
use db::{PostgresSource, ConnectionSource};
use std::error;
use std::error::Error;
use std::fmt;
use std::marker::{Send, Sync};
use std::convert::From;
use web3;

pub trait PostgresHelper: Clone + Send + Sync {
    fn query<T: TryFromRow>(
        &mut self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, CambioError>;
    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, CambioError>;
    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, CambioError>;
}

#[derive(Clone)]
pub struct PostgresHelperImpl {
    conn_source: PostgresSource,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CambioError {
    desc: String,
}

impl CambioError {
    pub fn new(desc: &str) -> Self {
        Self { desc: desc.to_owned() }
    }
}

impl error::Error for CambioError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for CambioError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBHelperError: {}", self.description())
    }
}

impl From<web3::Error> for CambioError {
    fn from(err: web3::Error) -> CambioError {
        CambioError::new(&format!("Error completing web3 operation: {:?}", err))
    }
}

impl PostgresHelperImpl {
    pub fn new(conn_source: PostgresSource) -> Self {
        PostgresHelperImpl { conn_source: conn_source }
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
        match connection.query(query, params) {
            Ok(query_result) => {
                let mut result_objs = Vec::new();
                for row in query_result.iter() {
                    match T::try_from_row(&row) {
                        Ok(obj) => result_objs.push(obj),
                        Err(error) => {
                            let error_message = format!("Error serialialising row. {}", error);
                            return Err(CambioError::new(&error_message));
                        }
                    }
                }
                Ok(result_objs)
            }
            Err(error) => {
                let msg = format!("Error while running query: {}", error.description());
                return Err(CambioError::new(&msg));
            }
        }
    }

    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, CambioError> {
        let conn = try!(self.conn_source.get());
        let err_func = |err| CambioError::new(&format!("Error running query: {}", err));
        conn.query(query, params).map_err(err_func)
    }

    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, CambioError> {
        let conn = try!(self.conn_source.get());
        let err_func = |err| CambioError::new(&format!("Error running query: {}", err));
        conn.execute(query, params).map_err(err_func)
    }
}

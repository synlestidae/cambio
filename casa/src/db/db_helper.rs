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

pub trait PostgresHelper: Clone + Send + Sync {
    fn query<T: TryFromRow>(
        &mut self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, PostgresHelperError>;
    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, PostgresHelperError>;
    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, PostgresHelperError>;
}

#[derive(Clone)]
pub struct PostgresHelperImpl {
    conn_source: PostgresSource,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PostgresHelperError {
    desc: String,
}

impl PostgresHelperError {
    pub fn new(desc: &str) -> Self {
        Self { desc: desc.to_owned() }
    }
}

impl error::Error for PostgresHelperError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for PostgresHelperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DBHelperError: {}", self.description())
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
    ) -> Result<Vec<T>, PostgresHelperError> {
        debug!("Getting connection");
        let mut connection = try!(self.conn_source.get());
        debug!("Running query {}", query);
        match connection.query(query, params) {
            Ok(query_result) => {
                debug!("Converting results");
                let mut result_objs = Vec::new();
                for row in query_result.iter() {
                    match T::try_from_row(&row) {
                        Ok(obj) => result_objs.push(obj),
                        Err(_) => return Err(PostgresHelperError::new("Error serialialising row")),
                    }
                }
                Ok(result_objs)
            }
            Err(error) => {
                let msg = format!("Error while running query: {}", error.description());
                println!("ERROR {:?}", error);
                return Err(PostgresHelperError::new(&msg));
            }
        }
    }

    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, PostgresHelperError> {
        let conn = try!(self.conn_source.get());
        let err_func = |err| PostgresHelperError::new(&format!("Error running query: {}", err));
        conn.query(query, params).map_err(err_func)
    }

    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, PostgresHelperError> {
        let conn = try!(self.conn_source.get());
        let err_func = |err| PostgresHelperError::new(&format!("Error running query: {}", err));
        conn.execute(query, params).map_err(err_func)
    }
}

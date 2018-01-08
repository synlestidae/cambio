use postgres::rows::{Rows};
use postgres::Connection;
use postgres::types::ToSql;
use postgres;
use db::try_from_row::TryFromRow;
use db::user_repository::UserRepository;
use std::error;
use std::error::Error;
use std::fmt;

pub trait PostgresHelper {
    fn query<T: TryFromRow>(
        &self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, PostgresHelperError>;
    fn execute(&self, query: &str, params: &[&ToSql]) -> postgres::Result<u64>;
    fn query_raw(&self, query: &str, params: &[&ToSql]) -> postgres::Result<Rows>;
}

pub struct PostgresHelperImpl {
    connection: Connection,
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
    pub fn new(conn: Connection) -> PostgresHelperImpl {
        PostgresHelperImpl { connection: conn }
    }
}

impl PostgresHelper for PostgresHelperImpl {
    fn query<T: TryFromRow>(
        &self,
        query: &str,
        params: &[&ToSql],
    ) -> Result<Vec<T>, PostgresHelperError> {
        match self.connection.query(query, params) {
            Ok(query_result) => {
                let mut result_objs = Vec::new();
                for row in query_result.iter() {
                    match T::try_from_row(&row) {
                        Ok(obj) => result_objs.push(obj),
                        Err(_) => return Err(PostgresHelperError::new("Error serialialising row")),
                    }
                }
                Ok(result_objs)
            },
            Err(error) => {
                let msg = format!("Error while running query: {}", error.description());
                return Err(PostgresHelperError::new(&msg));
            }
        }
    }

    fn query_raw(&self, query: &str, params: &[&ToSql]) -> postgres::Result<Rows> {
        self.connection.query(query, params)
    }

    fn execute(&self, query: &str, params: &[&ToSql]) -> postgres::Result<u64> {
        self.connection.execute(query, params)
    }
}

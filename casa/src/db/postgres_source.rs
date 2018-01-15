use r2d2_postgres::{TlsMode, PostgresConnectionManager};
use r2d2;
use r2d2::{Pool, PooledConnection, ManageConnection};
use std::io;
use postgres::Connection;
use std::ops::Deref;
use db::{ConnectionSource, PostgresHelperError, PostgresPooledConn};

#[derive(Clone)]
pub struct PostgresSource {
    pool: Pool<PostgresConnectionManager>,
}

impl ConnectionSource for PostgresSource {
    fn get<'a>(&'a mut self) -> Result<PostgresPooledConn, PostgresHelperError> {
        match self.pool.get() {
            Ok(pooled_connection) => {
                let connection: PooledConnection<PostgresConnectionManager> = pooled_connection;
                Ok(connection)
            }
            Err(error) => {
                let helper_error = PostgresHelperError::new(
                    &format!("Failed to get database connection: {}", error),
                );
                Err(helper_error)
            }
        }
    }
}

impl PostgresSource {
    pub fn new(connection_str: &str) -> io::Result<Self> {
        let manager_result = PostgresConnectionManager::new(connection_str, TlsMode::None);

        if let Err(error) = manager_result {
            return Err(io::Error::new(io::ErrorKind::Other, error));
        }

        match r2d2::Pool::new(manager_result.unwrap()) {
            Ok(pool) => Ok(PostgresSource { pool: pool }),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, error)),
        }
    }
}

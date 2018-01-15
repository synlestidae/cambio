use postgres::Connection;
use r2d2::{PooledConnection};
use r2d2_postgres::PostgresConnectionManager;
use db::{PostgresHelperError};

pub type PostgresPooledConn = PooledConnection<PostgresConnectionManager>;

pub trait ConnectionSource: Clone {
    fn get<'a>(&'a mut self) -> Result<PostgresPooledConn, PostgresHelperError>;
}

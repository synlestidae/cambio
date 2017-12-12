use postgres::rows::Row;
use postgres::Connection;
use postgres::types::ToSql;
use postgres;
use db::row_convert_err::RowConvertErr;
use db::try_from_row::TryFromRow;
use std::error::Error;
use std::error;
use std::fmt;

pub trait PostgresHelper {
   fn query<T: TryFromRow>(&self, query: &str, params: &[&ToSql]) -> 
       Result<Vec<T>, PostgresHelperError>;
   fn execute(&self, query: &str, params: &[&ToSql]) -> postgres::Result<u64>;
}

pub struct PostgresHelperImpl {
    connection: Connection
}



#[derive(Debug, Clone)]
pub struct PostgresHelperError {
    desc: String
}

impl PostgresHelperError  {
    fn new(desc: &str) -> Self {
        Self {
            desc: desc.to_owned()
        }
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
        PostgresHelperImpl {
            connection: conn
        }
    }
}

impl PostgresHelper for PostgresHelperImpl {
   fn query<T: TryFromRow>(&self, query: &str, params: &[&ToSql]) -> 
       Result<Vec<T>, PostgresHelperError> 
   {
       match (self.connection.transaction(), self.connection.query(query, params)) {
           (Ok(transaction), Ok(query_result)) => {
               let mut result_objs = Vec::new();
               for row in query_result.iter() {
                   match T::try_from_row(&row) {
                       Ok(obj) => result_objs.push(obj),
                       Err(_) => return Err(PostgresHelperError::new("Error serialialising row"))
                   }
               }
               transaction.commit();
               Ok(result_objs)
           },
           (Err(_), _) => {
               // is an error trying to start the transaction
               unimplemented!();
           },
           (_, Err(_)) => {
               // an error with getting the query
               unimplemented!();
           }
       }
   }

   fn execute(&self, query: &str, params: &[&ToSql]) -> postgres::Result<u64> {
       self.connection.execute(query, params)
   }
}

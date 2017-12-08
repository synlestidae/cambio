use postgres::rows::Row;
use postgres::Connection;
use postgres::types::ToSql;
use postgres;
use db::row_convert_err::RowConvertErr;
use db::try_from::TryFrom;

pub trait PostgresHelper {
   fn query<'a, T: TryFrom<&'a Row<'a>>>(&self, query: &str, params: &[&ToSql]) -> 
       Result<Vec<T>, PostgresHelperError>;
   fn execute(&self, query: &str, params: &[&ToSql]) -> postgres::Result<u64>;
}

pub struct PostgresHelperImpl {
    connection: Connection
}


pub struct PostgresHelperError; 

impl PostgresHelperImpl {
    pub fn new(conn: Connection) -> PostgresHelperImpl {
        PostgresHelperImpl {
            connection: conn
        }
    }
}

impl PostgresHelper for PostgresHelperImpl {
   fn query<'a, T: TryFrom<&'a Row<'a>>>(&self, query: &str, params: &[&ToSql]) -> 
       Result<Vec<T>, PostgresHelperError> 
   {
       match (self.connection.transaction(), self.connection.query(query, params)) {
           (Ok(transaction), Ok(query_result)) => {
               let mut result_objs = Vec::new();
               for row in query_result.iter() {
                   match T::try_from(&row) {
                       Ok(obj) => result_objs.push(obj),
                       Err(_) => return Err(PostgresHelperError)
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

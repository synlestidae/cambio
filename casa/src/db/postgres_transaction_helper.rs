use postgres::transaction::Transaction as PTransaction;
use db::Transaction;
use db::PostgresHelper;
use db::TryFromRow;
use db::CambioError;
use postgres::rows::Rows;
use postgres::types::ToSql;

pub struct PostgresTransactionHelper<'a> {
    transaction: PTransaction<'a>
}

impl<'a> PostgresTransactionHelper<'a> {
    pub fn new(tx: PTransaction<'a>) -> Self {
       Self {
           transaction: tx
       }
    }
}

impl<'a> PostgresHelper for PostgresTransactionHelper<'a> {
    fn query<T: TryFromRow>(&mut self, query: &str, params: &[&ToSql]) -> Result<Vec<T>, CambioError> {
        let connection = &mut self.transaction;
        let rows = try!(connection.query(query, params));

        let mut result_objs = Vec::new();
        for row in rows.iter() {
            let obj = try!(T::try_from_row(&row));
            result_objs.push(obj);
        }
        Ok(result_objs)
    }

    fn query_raw(&mut self, query: &str, params: &[&ToSql]) -> Result<Rows, CambioError> {
        let conn = self.transaction;
        let result = try!(conn.query(query, params));
        Ok(result)
    }

    fn execute(&mut self, query: &str, params: &[&ToSql]) -> Result<u64, CambioError> {
        let conn = self.transaction;
        let result = try!(conn.execute(query, params));
        Ok(result)
    }
}

impl<'a> Transaction<'a> for PostgresTransactionHelper<'a> {
    fn commit(self) -> Result<(), CambioError> {
        self.transaction.set_commit();
        Ok(try!(self.transaction.commit()))
    }
    fn rollback(self) -> Result<(), CambioError> {
        self.transaction.set_rollback();
        Ok(try!(self.transaction.finish()))
    }
}

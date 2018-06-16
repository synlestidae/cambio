use db::Transaction;
use db::CambioError;
use db::PostgresHelper;

pub trait TransactionSource<'a, T: Transaction<'a> + PostgresHelper> {
    fn begin_transaction(&'a mut self) -> Result<T, CambioError>;
}

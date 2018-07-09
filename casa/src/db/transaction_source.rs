use db::Transaction;
use db::CambioError;
use db::PostgresHelper;

pub trait TransactionSource<T: Transaction + PostgresHelper> {
    fn begin_transaction(&mut self) -> Result<T, CambioError>;
}

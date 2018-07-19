use db::CambioError;
use db::PostgresHelper;
use db::Transaction;

pub trait TransactionSource<T: Transaction + PostgresHelper> {
    fn begin_transaction(&mut self) -> Result<T, CambioError>;
}

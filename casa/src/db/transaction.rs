use db::CambioError;

pub trait Transaction {
    fn commit(self) -> Result<(), CambioError>;
    fn rollback(self) -> Result<(), CambioError>;
}

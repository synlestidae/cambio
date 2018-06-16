use db::CambioError;

pub trait Transaction<'a> {
    fn commit(self) -> Result<(), CambioError>;
    fn rollback(self) -> Result<(), CambioError>;
}

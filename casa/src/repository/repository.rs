use repository::clause::Clause;
use db::CambioError;

pub type ItemResult<T> = Result<T, CambioError>;
pub type VecResult<T> = Result<Vec<T>, CambioError>;

pub trait Repository {
    type Item;
    type Clause: Clause;

    fn read(&mut self, clause: &Self::Clause) -> VecResult<Self::Item>;
    fn create(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
    fn update(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
    fn delete(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
}

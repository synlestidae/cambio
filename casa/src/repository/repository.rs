use repository::clause::Clause;
use db::CambioError;

pub type ItemResult<T> = Result<T, CambioError>;
pub type VecResult<T> = Result<Vec<T>, CambioError>;

pub trait Repository {
    type Item;
    type Clause: Clause;

    fn create(item: &Self::Item) -> ItemResult<Self::Item>;
    fn read(clause: &Self::Clause) -> VecResult<Self::Item>;
    fn update(item: &Self::Item) -> ItemResult<Self::Item>;
    fn delete(item: &Self::Item) -> ItemResult<Self::Item>;
}

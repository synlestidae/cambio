use db::CambioError;

pub type ItemResult<T> = Result<T, CambioError>;
pub type VecResult<T> = Result<Vec<T>, CambioError>;

pub trait RepoRead {
    type Item;
    type Clause;
    fn read(&mut self, clause: &Self::Clause) -> VecResult<Self::Item>;
}

pub trait RepoCreate {
    type Item;
    fn create(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
}

pub trait RepoUpdate {
    type Item;
    fn update(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
}

pub trait RepoDelete {
    type Item;
    fn delete(&mut self, item: &Self::Item) -> ItemResult<Self::Item>;
}

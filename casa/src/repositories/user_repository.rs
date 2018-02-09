use repository;
use db;
use domain;

#[derive(Clone)]
pub struct UserRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> repository::Repository for UserRepository<T> {
    type Item = domain::User;
    type Clause = repository::GeneralClause;

    fn read(clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        unimplemented!()
    }

    fn create(item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn update(item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn delete(item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }
}

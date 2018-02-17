use repository;
use db;
use domain;

#[derive(Clone)]
pub struct MediaRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> MediaRepository<T> {
    pub fn new(db: T) -> Self {
        MediaRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::Repository for MediaRepository<T> {
    type Item = domain::StoredMedia;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        unimplemented!()
    }

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }
}

use repository;
use db;
use domain;
use postgres::types::ToSql;

#[derive(Clone)]
pub struct PaymentRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> PaymentRepository<T> {
    pub fn new(db: T) -> Self {
        PaymentRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::Repository for PaymentRepository<T> {
    type Item = domain::Payment;
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

use domain;
use repository;

pub enum UserClause {
    Id(domain::Id),
    EmailAddress(String)
}

impl repository::Clause for UserClause {
}

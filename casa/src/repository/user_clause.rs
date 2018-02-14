use domain;
use repository;

#[derive(Debug, Eq, PartialEq)]
pub enum UserClause {
    All(bool),
    Id(domain::Id),
    EmailAddress(String),
    SessionToken(String),
    UniqueId(String)
}

impl repository::Clause for UserClause {
}

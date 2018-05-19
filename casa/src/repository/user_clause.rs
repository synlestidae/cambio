use domain;
use repository;

#[derive(Debug, Eq, PartialEq)]
pub enum UserClause {
    All(bool), //bool -> only_active
    Id(domain::Id),
    EmailAddress(String),
    SessionToken(String),
    UniqueId(String),
}

impl repository::Clause for UserClause {}

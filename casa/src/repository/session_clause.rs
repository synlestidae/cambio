use domain;
use repository;

pub enum SessionClause {
    Id(domain::Id),
    EmailAddress(String),
    SessionToken(String)
}

impl repository::Clause for SessionClause {}

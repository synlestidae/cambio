use domain;
use repository;

pub enum AccountClause {
    Id(domain::Id),
    EmailAddress(String)
}

impl repository::Clause for AccountClause {
}

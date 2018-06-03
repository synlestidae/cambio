use std::error;
use query::SelectSpec;

pub trait Selectable<E> {
    fn get_specifier(&self) -> SelectSpec;
}

use query;
use domain;

impl Selectable<domain::User> for domain::UserId {
    fn get_specifier(&self) -> SelectSpec {
        SelectSpec {
            table: query::TableName::parse("users").unwrap(),
            columns: vec![query::Column::All],
            clause: Some(query::BoolClause::id_equals(self.0))
        }
    }
}

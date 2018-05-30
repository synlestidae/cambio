use std::error;
use query::SelectSpec;

pub trait Selectable<E> {
    fn get_specifier(&self) -> SelectSpec;
}

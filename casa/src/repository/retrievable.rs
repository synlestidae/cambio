use db::CambioError;
use db::PostgresHelper;
use repository::Clause;
use repository::RepoRead;

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c

pub trait Retrievable {
    type Item;

    fn get<H: PostgresHelper>(db: &mut H) -> Result<Self::Item, CambioError>; 
    fn get_option<H: PostgresHelper>(db: &mut H)  -> Result<Option<Self::Item>, CambioError>;
}

use db::CambioError;
use db::PostgresHelper;
use repository::Clause;
use repository::RepoRead;

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c

pub trait Retrievable<Item> {
    fn get<H: PostgresHelper>(&self, db: H) -> Result<Item, CambioError>; 
    fn get_option<H: PostgresHelper>(&self, db: H)  -> Result<Option<Item>, CambioError>;
}

use db::CambioError;
use db::PostgresHelper;
use repository::UserClause;
use repository::RepoRead;
use repositories;
use repository;
use domain;

// suppose I just want an easy way to retrieve the owner id from the user
// then i implement retrievable where the Item is a User, the c

pub trait Retrievable<Item> {
    fn get<H: PostgresHelper>(&self, mut db: H) -> Result<Item, CambioError>; 
    fn get_option<H: PostgresHelper>(&self, mut db: H)  -> Result<Option<Item>, CambioError>;
}

impl Retrievable<domain::User> for domain::OrderId {
    fn get<H: PostgresHelper>(&self, mut db: H) -> Result<domain::User, CambioError> {
        unimplemented!()
    }

    fn get_option<H: PostgresHelper>(&self, mut db: H)  -> Result<Option<domain::User>, CambioError> {
        unimplemented!()
    }
}

impl Retrievable<domain::Session> for domain::SessionToken {
    fn get<H: PostgresHelper>(&self, mut db: H) -> Result<domain::Session, CambioError> {
        match self.get_option(db) {
            Ok(Some(session_token)) => Ok(session_token),
            Ok(None) => Err(CambioError::unauthorised()),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, mut db: H)  -> Result<Option<domain::Session>, CambioError> {
        let clause = repository::UserClause::SessionToken(self.0.to_owned());
        let mut session_repo = repositories::SessionRepository::new(db);
        session_repo.read(&clause).map(|mut s| s.pop())
    }
}


impl Retrievable<domain::User> for domain::OwnerId {
    fn get<H: PostgresHelper>(&self, mut db: H) -> Result<domain::User, CambioError> {
        match self.get_option(db) {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(CambioError::not_found_search("User could not be located.", 
                "User with owner ID not found")),
            Err(err) => Err(err)
        }
    }

    fn get_option<H: PostgresHelper>(&self, mut db: H)  -> Result<Option<domain::User>, CambioError> {
        let mut matches = try!(db.query(SELECT_BY_OWNER, &[&self.0]));
        Ok(matches.pop())
    }
}

const SELECT_BY_OWNER: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE account_owner.id = $1";

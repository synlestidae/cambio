use repository;
use db;
use domain;

#[derive(Clone)]
pub struct UserRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> repository::Repository for UserRepository<T> {
    type Item = domain::User;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        match clause {
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::UserClause::EmailAddress(ref email_address) => self.db_helper.query(SELECT_BY_EMAIL,
                &[email_address])
        }
    }

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let err = db::CambioError::shouldnt_happen("Failed to locate account after creating it", 
            "Error during user creation");
        self.db_helper.execute(INSERT, &[&item.email_address, &item.password_hash]);
        let mut users = try!(self.read(&repository::UserClause::EmailAddress(item.email_address.clone())));
        match users.pop() {
            Some(user) => Ok(user),
            None => Err(err)
        }
    }

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        if item.id.is_none() {
            return Err(db::CambioError::format_obj("Cannot update a user without an ID", "Update obj ID is None"));
        }
        let id = item.id.unwrap();
        //try!(self.db_helper.execute());
        self.read(&repository::UserClause::Id(id));
        unimplemented!()
    }

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }
}

const SELECT_BY_ID: &'static str = "SELECT *, id as user_id FROM users WHERE id = $1";
const SELECT_BY_EMAIL: &'static str = "SELECT *, id as user_id FROM users WHERE email_address = $1";

const INSERT: &'static str = "INSERT INTO users(email_address, password_hash) VALUES($1, $2)";

const UPDATE_BY_ID: &'static str = "SELECT *, id as user_id FROM users WHERE id = $1";
const UPDATE_BY_EMAIL: &'static str = "SELECT *, id as user_id FROM users WHERE email_address = $1";

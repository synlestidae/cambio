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
        let result = if let Some(ref id) = item.id {
            self.db_helper.execute(UPDATE_BY_ID, &[id,  &item.email_address, &item.password_hash])
        } else {
            self.db_helper.execute(UPDATE_BY_EMAIL, &[&item.email_address, &item.password_hash])
        };
        try!(result);
        Ok(item.clone())
    }

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        Err(db::CambioError::shouldnt_happen(
            "Cannot remove user from database", 
            "DELETE for users table not supported"
        ))
    }
}

const SELECT_BY_ID: &'static str = "SELECT *, id as user_id FROM users WHERE id = $1";
const SELECT_BY_EMAIL: &'static str = "SELECT *, id as user_id FROM users WHERE email_address = $1";

const INSERT: &'static str = "INSERT INTO users(email_address, password_hash) VALUES($1, $2)";

const UPDATE_BY_ID: &'static str = "UPDATE users SET email_address = $2, password_hash = $3 WHERE id = $1 LIMIT 1";
const UPDATE_BY_EMAIL: &'static str = "UPDATE users password_hash = $3 WHERE email_address = $1 LIMIT 1";

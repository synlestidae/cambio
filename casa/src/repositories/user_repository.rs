use repository;
use db;
use domain;
use checkmail;

#[derive(Clone)]
pub struct UserRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> UserRepository<T> {
    pub fn new(db: T) -> Self {
        UserRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::Repository for UserRepository<T> {
    type Item = domain::User;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        match clause {
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::UserClause::EmailAddress(ref email_address) => self.db_helper.query(SELECT_BY_EMAIL,
                &[email_address]),
            &repository::UserClause::All(_) => self.db_helper.query(SELECT_ALL, &[]),
            _ => Err(db::CambioError::shouldnt_happen("Invalid query to get account", 
                    &format!("Clause {:?} not supported by AccountRepository", clause)))
        }
    }

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        if !checkmail::validate_email(&item.email_address) {
            return Err(db::CambioError::bad_input("Invalid email format", "Invalid email format"));
        }

        let err = db::CambioError::shouldnt_happen("Failed to locate account after creating it", 
            "Error during user creation");
        let rows_affected = try!(self.db_helper.execute(INSERT, &[&item.email_address, &item.password_hash]));
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

const SELECT_BY_ID: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE users.id = $1";

const SELECT_ALL: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id";

const SELECT_BY_EMAIL: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE users.email_address = $1";

const INSERT: &'static str = "SELECT register_user($1, $2)";
const UPDATE_BY_ID: &'static str = "UPDATE users SET email_address = $2, password_hash = $3 WHERE id = $1";
const UPDATE_BY_EMAIL: &'static str = "UPDATE users password_hash = $3 WHERE email_address = $1";

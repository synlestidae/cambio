use checkmail;
use db;
use domain;
use repository;
use repository::*;

#[derive(Clone)]
pub struct UserRepository<T: db::PostgresHelper> {
    db_helper: T,
}

impl<T: db::PostgresHelper> UserRepository<T> {
    pub fn new(db: T) -> Self {
        UserRepository { db_helper: db }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for UserRepository<T> {
    type Item = domain::User;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        unimplemented!()
    }
}

impl<T: db::PostgresHelper> repository::RepoCreate for UserRepository<T> {
    type Item = domain::User;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }
}

impl<T: db::PostgresHelper> repository::RepoUpdate for UserRepository<T> {
    type Item = domain::User;

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        unimplemented!()
    }
}

impl<T: db::PostgresHelper> repository::RepoDelete for UserRepository<T> {
    type Item = domain::User;

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        Err(db::CambioError::shouldnt_happen(
            "Cannot remove user from database",
            "DELETE for users table not supported",
        ))
    }
}

const SELECT_BY_ID: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE users.id = $1";

const SELECT_BY_OWNER: &'static str = "
    SELECT *, users.id as user_id, account_owner.id as owner_id
    FROM users 
    JOIN account_owner ON account_owner.user_id = users.id 
    WHERE account_owner.id = $1";

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
const UPDATE_BY_ID: &'static str =
    "UPDATE users SET email_address = $2, password_hash = $3 WHERE id = $1";
const UPDATE_BY_EMAIL: &'static str = "UPDATE users password_hash = $3 WHERE email_address = $1";

use db::{PostgresHelper, CambioError};
use domain::{User, Session, Id};
use repository::Repository;
use repositories;
use repository;
use std::error::Error;
use bcrypt::hash;
use checkmail;

#[derive(Clone)]
pub struct UserService<T: PostgresHelper> {
    user_repository: repositories::UserRepository<T>,
    session_repository: repositories::SessionRepository<T>,
}

const BCRYPT_COST: u32 = 8;

impl<T: PostgresHelper> UserService<T> {
    pub fn new(db_helper: T) -> Self {
        let users = repositories::UserRepository::new(db_helper.clone());
        let sessions = repositories::SessionRepository::new(db_helper.clone());
        Self { 
            user_repository: users,
            session_repository: sessions
        }
    }

    pub fn register_user(
        &mut self,
        email_address: &str,
        password: String,
    ) -> Result<User, CambioError> {
        if !checkmail::validate_email(&email_address.to_owned()) {
            return Err(CambioError::bad_input("Please check that the email entered is valid", "Email address is invalid"));
        }


        let query = repository::UserClause::EmailAddress(email_address.to_owned());
        // check user exists
        if let Some(_) = try!(self.user_repository.read(&query)).pop() {
            return Err(CambioError::user_exists());
        }

        // get the BCrypt hash
        let password_hash = try!(hash(&password, BCRYPT_COST));

        drop(password);

        let mut user = User {
            id: None,
            email_address: email_address.to_owned(),
            password: None,
            password_hash: Some(password_hash)
        };

        user = try!(self.user_repository.create(&user));
        Ok(user)
    }

    pub fn log_user_in(&mut self, email_address: &str, password: String) 
        -> Result<Session, CambioError> {
        let query = repository::UserClause::EmailAddress(email_address.to_owned());
        let user_option = try!(self.user_repository.read(&query)).pop();
        if user_option.is_none() {
            return Err(CambioError::not_found_search("Could not account for that email", 
                "User repository returned None for User"));
        }
        let user = user_option.unwrap();
        if !user.hash_matches_password(&password) {
            return Err(CambioError::invalid_password());
        }

        drop(password);

        let mut session = Session::new(email_address, SESSION_TIME_MILLISECONDS);
        let session = try!(self.session_repository.create(&session));
        Ok(session)
    }

    pub fn log_user_out(&mut self, email_address: &str) -> Result<(), CambioError> {
        let query = repository::SessionClause::EmailAddress(email_address.to_owned());
        let sessions = try!(self.session_repository.read(&query));
        for session in sessions.into_iter() {
            session.session_state = SessionState::Invalidated;
            try!(self.session_repository.update(&session));
        }
        Ok(())
    }
}

const SESSION_TIME_MILLISECONDS: i32 = 1000 * 60 * 15;

const GET_USER_QUERY: &'static str = "SELECT id, email_address, password_hash FROM users WHERE email_address = $1";
const GET_USER_QUERY_ID: &'static str = "SELECT id, email_address, password_hash FROM users WHERE id = $1";
const ACTIVATE_USER_SESSION_QUERY: &'static str = "SELECT * FROM activate_user_session($1)";
const GET_SESSION_QUERY: &'static str = "SELECT users.email_address, session_info.session_token, session_info.started_at, session_info.ttl_milliseconds
    FROM users, session_info
    WHERE users.email_address = $1 AND session_info.session_token = $2 AND session_info.session_state = 'valid' AND
        now() at time zone 'utc' < session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL))";

const LOG_USER_OUT_QUERY: &'static str = "UPDATE session_info 
    SET session_state = 'invalidated'
    FROM user_session
    JOIN users ON users.id = user_session.user_id
    WHERE users.email_address = $1";

const REGISTER_USER: &'static str = "SELECT register_user($1, $2);";


const GET_OWNER_QUERY: &'static str = "
    SELECT account_owner.id AS owner_id FROM account_owner, users 
    WHERE account_owner.user_id = users.id AND users.email_address = $1";

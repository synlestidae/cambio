use db::{PostgresHelper, CambioError, ErrorKind, ErrorReccomendation};
use domain::{User, Session, Id};
use repository::Repository;
use repositories::UserRepository;
use repository;
use std::error::Error;
use bcrypt::hash;
use checkmail;

#[derive(Clone)]
pub struct UserService<T: PostgresHelper> {
    db_helper: T,
    user_repository: UserRepository<T>
}

const BCRYPT_COST: u32 = 8;

impl<T: PostgresHelper> UserService<T> {
    pub fn new(db_helper: T) -> Self {
        Self { 
            db_helper: db_helper.clone(),
            user_repository: UserRepository::new(db_helper.clone())
        }
    }

    pub fn get_user_by_email(
        &mut self,
        email_address: &str,
    ) -> Result<Option<User>, CambioError> {
        let e = email_address.to_owned();
        self.get(&repository::UserClause::EmailAddress(e))
    }

    pub fn get_user(
        &mut self,
        user_id: Id,
    ) -> Result<Option<User>, CambioError> {
        self.get(&repository::UserClause::Id(user_id))
    }

    fn get(&mut self, c: &repository::UserClause) -> Result<Option<User>, CambioError> {
        let mut matches = try!(self.user_repository.read(c));
        Ok(matches.pop())
    }

    pub fn register_user(
        &mut self,
        email_address: &str,
        password: String,
    ) -> Result<User, CambioError> {
        if !checkmail::validate_email(&email_address.to_owned()) {
            return Err(CambioError::bad_input("Please check that the email entered is valid", "Email address is invalid"));
        }

        // check user exists
        if let Some(_) = try!(self.get_user_by_email(email_address)) {
            return Err(CambioError::user_exists());
        }

        // get the BCrypt hash
        let password_hash = try!(hash(&password, BCRYPT_COST));

        drop(password);

        try!(self.db_helper.execute(
            REGISTER_USER,
            &[&email_address, &password_hash],
        ));

        match try!(self.get_user_by_email(email_address)) {
            Some(user) => Ok(user),
            None => Err(CambioError::shouldnt_happen(
                "Couldn't find your account after registering",
                "Failed to find account by email after registration"))
        }
    }

    pub fn log_user_in(&mut self, email_address: &str, password: String) 
        -> Result<Option<Session>, CambioError> {
        let user_option = try!(self.get_user_by_email(email_address));
        if user_option.is_none() {
            return Ok(None);
        }
        let user = user_option.unwrap();
        if !user.hash_matches_password(&password) {
            return Err(CambioError::invalid_password());
        }
        drop(password);

        // code from here is AUTHORISED
        let query_result = self.db_helper.query_raw(
            ACTIVATE_USER_SESSION_QUERY,
            &[&email_address],
        );
        let rows = try!(query_result);
        let row = rows.get(0);
        let session_token_option: Option<String> = row.get(0);
        match session_token_option {
            None => Ok(None),
            Some(session_token) => self.get_existing_session(email_address, &session_token)
        }
    }

    pub fn get_existing_session(
        &mut self,
        email_address: &str,
        session_token: &str) -> Result<Option<Session>, CambioError> {
        let session = try!(self.db_helper.query(
            GET_SESSION_QUERY,
            &[&email_address, &session_token],
        )).pop();
        Ok(session)
    }

    pub fn log_user_out(&mut self, email_address: &str) -> Result<(), CambioError> {
        try!(self.db_helper.execute(LOG_USER_OUT_QUERY, &[&email_address]));
        Ok(())
    }

    pub fn get_owner_id_by_email_address(
        &mut self,
        email_address: &str,
    ) -> Result<Id, CambioError> {
        let rows = try!(self.db_helper.query_raw(GET_OWNER_QUERY, &[&email_address]));
        if rows.len() >= 1 {
            let row = rows.get(0);
            Ok(row.get("owner_id"))
        } else {
            Err(CambioError::new("Failed to load accounts for user", 
                &format!("Owner ID not found for {}", email_address),
                ErrorKind::NotFound,
                ErrorReccomendation::CheckInput))
        }
    }
}

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

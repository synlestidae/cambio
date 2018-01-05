use db::{PostgresHelper, PostgresHelperError};
use domain::{User, Session};
use std::error::Error;

pub struct UserRepository<T: PostgresHelper> {
    db_helper: T
}

const GET_USER_QUERY: &'static str = "SELECT email_address, password_hash FROM users WHERE email_address = $1";
const ACTIVATE_USER_SESSION_QUERY: &'static str = "SELECT session_token FROM activate_user_session($1)";
const GET_SESSION_QUERY: &'static str = 
"SELECT users.email_address, session_info.session_token, session_info.started_at, session_info.ttl_milliseconds
    WHERE users.email_address = $1 AND session_info.session_token = $2 AND session_info.session_state = 'valid'";

const LOG_USER_OUT_QUERY: &'static str = "UPDATE 
    session_info si
    JOIN user_session ON user_session.session_info_id = session_info.id
    JOIN users ON users.id = user_session.user_id
    SET si.session_state = 'invalidated'
    WHERE users.email_address = $1";

impl<T: PostgresHelper> UserRepository<T> {
    pub fn new(db_helper: T) -> Self {
        UserRepository {
            db_helper: db_helper
        }
    }

    pub fn get_user_by_email(&mut self, email_address: &str) -> Result<Option<User>, PostgresHelperError> {
        match self.db_helper.query(GET_USER_QUERY, &[&email_address]) {
            Ok(mut users) => Ok(users.pop()),
            Err(err) => Err(PostgresHelperError::new(err.description()))
        }
    }

    pub fn log_user_in(&mut self, email_address: &str, password: String) 
            -> Result<Option<Session>, PostgresHelperError> {
        let user_option = try!(self.get_user_by_email(email_address));
        if user_option.is_none() {
            return Ok(None);
        }
        let user = user_option.unwrap();
        if !user.hash_matches_password(&password) {
            return Err(PostgresHelperError::new("Password does not match hash"));
        }
        drop(password);

        // code from here is AUTHORISED

        let query_result = self.db_helper.query_raw(ACTIVATE_USER_SESSION_QUERY, &[&email_address]); 
        if let Err(query_err) = query_result {
            return Err(PostgresHelperError::new(query_err.description()));
        }
        let rows = query_result.unwrap();
        let row = rows.get(0);
        let session_token_option: Option<String> = row.get(0);
        match session_token_option {
            None => Ok(None),
            Some(session_token) => self.get_existing_session(email_address, &session_token)
        }
    }

    pub fn get_existing_session(&mut self, email_address: &str, session_token: &str) -> Result<Option<Session>, PostgresHelperError> {
        let mut sessions: Vec<Session> = try!(self.db_helper.query(GET_SESSION_QUERY, &[&email_address, &session_token]));
        Ok(sessions.pop())
    }

    pub fn log_user_out(&mut self, email_address: &str) -> Result<(), PostgresHelperError> {
        if let Err(error) = self.db_helper.execute(LOG_USER_OUT_QUERY, &[]) {
            return Err(PostgresHelperError::new(&format!("Error logging user out: {}", error.description())));
        }
        Ok(())
    }
}

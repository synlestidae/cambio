use repository;
use db;
use domain;
use chrono::prelude::*;
use repository::*;

#[derive(Clone)]
pub struct SessionRepository<T: db::PostgresHelper> {
    db_helper: T
}

impl<T: db::PostgresHelper> SessionRepository<T> {
    pub fn new(db: T) -> Self {
        SessionRepository {
            db_helper: db
        }
    }
}

impl<T: db::PostgresHelper> repository::RepoRead for SessionRepository<T> {
    type Item = domain::Session;
    type Clause = repository::UserClause;

    fn read(&mut self, clause: &Self::Clause) -> repository::VecResult<Self::Item> {
        debug!("Retrieving with clause: {:?}", clause);
        match clause {
            &repository::UserClause::Id(ref id) => self.db_helper.query(SELECT_BY_ID, &[id]),
            &repository::UserClause::EmailAddress(ref email) => { 
                self.db_helper.query(SELECT_BY_EMAIL, &[email])
            },
            &repository::UserClause::SessionToken(ref token) => { 
                self.db_helper.query(SELECT_BY_TOKEN, &[token]) 
            },
            _ => Err(db::CambioError::shouldnt_happen("Invalid query to get user", 
                    &format!("Clause {:?} not supported by SessionRepository", clause)))
        }
    }
}

impl <T: db::PostgresHelper> repository::RepoCreate for SessionRepository<T> {
    type Item = domain::Session;

    fn create(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        if let Some(ref email) = item.email_address {
            let email_address = email.to_owned();
            try!(self.db_helper.execute(ACTIVATE, &[&email_address]));
            let c = repository::UserClause::EmailAddress(email_address);
            match try!(self.read(&c)).pop() {
                Some(session) => Ok(session),
                None => Err(db::CambioError::shouldnt_happen(
                    "Tried to log you in but couldn't find session",
                    "Could not retrieve session after activation"
                ))
            }
        } else {
            Err(db::CambioError::format_obj(
                "Tried to log you in but didn't have your email", 
                "Cannot create session when email_address = None")
            )
        }
    }
}

impl <T: db::PostgresHelper> repository::RepoUpdate for SessionRepository<T> {
    type Item = domain::Session;

    fn update(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let id = match item.id {
            Some(id) => {
                id
            },
            None => return Err(db::CambioError::format_obj(
                "Session doesn't exist in database", "Cannot
                update session without ID"))
        };
        let result = self.db_helper.execute(UPDATE, &[&id, 
            &item.session_token, 
            &item.started_at, 
            &item.ttl_milliseconds]);
        let update_error = db::CambioError::db_update_failed("Session");
        let rows = try!(result);
        if rows < 1 {
            return Err(update_error);
        }
        let session_result = try!(self.read(&repository::UserClause::Id(id))).pop();
        session_result.ok_or(update_error)
    }
}

impl <T: db::PostgresHelper> repository::RepoDelete for SessionRepository<T> {
    type Item = domain::Session;

    fn delete(&mut self, item: &Self::Item) -> repository::ItemResult<Self::Item> {
        let mut item_copy = item.clone();
        item_copy.session_state = domain::SessionState::Invalidated;
        self.update(&item_copy)
    }
}

const SELECT_BY_EMAIL: &'static str = "
    SELECT user_session.id AS session_id, session_info.*, users.email_address, users.id as user_id,
    FROM user_session
    JOIN session_info ON session_info.id = user_session.session_info_id
    JOIN users ON users.id = user_session.user_id
        WHERE users.email_address = $1 AND 
        (now() at time zone 'utc') < (session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)))";

const SELECT_BY_ID: &'static str = "
    SELECT user_session.id AS session_id, session_info.*, users.email_address,  users.id as user_id,
    FROM user_session
    JOIN session_info ON session_info.id = user_session.session_info_id
    JOIN users ON user_session.user_id = users.id
    WHERE user_session.id = $1 AND 
        (now() at time zone 'utc') < (session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)))";

const SELECT_BY_TOKEN: &'static str = "
    SELECT user_session.id AS session_id, session_info.*, users.email_address, users.id as user_id,
    FROM user_session
    JOIN session_info ON session_info.id = user_session.session_info_id
    JOIN users ON user_session.user_id = users.id
    WHERE session_info.session_token = $1 AND 
        (now() at time zone 'utc') < (session_info.started_at + (session_info.ttl_milliseconds * ('1 millisecond'::INTERVAL)))";

const ACTIVATE: &'static str = "SELECT activate_user_session($1)";

const UPDATE: &'static str = "UPDATE session_info SET 
    session_token=$2, started_at=$3, ttl_milliseconds=$4
    FROM user_session 
    JOIN users ON user_session.user_id = users.id
    WHERE user_session=$1 AND 
        user_session.session_info_id = session_info.id";

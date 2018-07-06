use bcrypt::hash;
use api::{RegistrationConfirm, PersonalDetails};
use checkmail;
use db::{CambioError, PostgresHelper};
use domain::{Id, Session, SessionState, User, Registration, Profile, Address, PersonalIdentity, EthAccount};
use repositories;
use repository;
use repository::*;
use services;
use std::error::Error;

pub struct UserService<T: PostgresHelper + Clone> {
    db: T,
    user_repository: repositories::UserRepository<T>,
    session_repository: repositories::SessionRepository<T>,
    web3_address: String
}

const BCRYPT_COST: u32 = 8;

impl<T: PostgresHelper + Clone> UserService<T> {
    pub fn new(db_helper: T, web3_address: &str) -> Self {
        let users = repositories::UserRepository::new(db_helper.clone());
        let sessions = repositories::SessionRepository::new(db_helper.clone());
        Self {
            db: db_helper.clone(),
            user_repository: users,
            session_repository: sessions,
            web3_address: web3_address.to_string()
        }
    }

    pub fn confirm_registration(&mut self, 
        registration: &Registration,
        personal_details: &PersonalDetails,
        eth_password: &str) -> Result<User, CambioError> {
        // TODO transaction needed here
        info!("Confirming registration");
        
        // Mark the registration as confirmed
        let mut confirmed_registration = registration.clone();
        confirmed_registration.confirm();
        try!(confirmed_registration.update(&mut self.db));
        info!("Creating registration...");

        // Create and store the user, ready to log in
        let user = try!(self.create_user(
            &confirmed_registration.email_address, 
            &confirmed_registration.password_hash,
            &personal_details,
            eth_password)
        );

        Ok(user)
    }

    pub fn register_user(
        &mut self,
        email_address: &str,
        password: &str,
        personal_details: &PersonalDetails) -> Result<User, CambioError> {
        // get the BCrypt hash
        let password_hash = try!(hash(password, BCRYPT_COST));
        self.create_user(email_address, &password_hash, personal_details, password)
    }

    pub fn create_user(
        &mut self,
        email_address: &str,
        password_hash: &str,
        personal_details: &PersonalDetails,
        eth_password: &str) -> Result<User, CambioError> {
        if !checkmail::validate_email(&email_address.to_owned()) {
            return Err(CambioError::bad_input(
                "Please check that the email entered is valid",
                "Email address is invalid",
            ));
        }

        let query = repository::UserClause::EmailAddress(email_address.to_owned());
        // check user exists
        if let Some(_) = try!(self.user_repository.read(&query)).pop() {
            return Err(CambioError::user_exists());
        }
        let mut user = User {
            id: None,
            email_address: email_address.to_owned(),
            password: None,
            password_hash: Some(password_hash.to_owned()),
            owner_id: None,
        };

        info!("Making a user");
        user = try!(self.user_repository.create(&user));
        info!("Made user {:?}", user.id);
        info!("Making eth account");
        try!(self.create_eth_accounts(email_address, eth_password));
        let profile = personal_details.clone().into_profile(user.id.unwrap());
        let new_profile = try!(profile.create(&mut self.db));
        info!("Making profile!");

        Ok(user)
    }

    fn create_eth_accounts(&mut self, 
        email_address: &str, 
        password: &str
    ) -> Result<EthAccount, CambioError> {
        info!("Creating ethereum account for {}", email_address);
        let mut eth_service = services::EthereumService::new(self.db.clone(), &self.web3_address);
        let account = try!(eth_service.new_account(email_address, password));
        info!("Eth account created. Saving...");
        let account_result = try!(account.create(&mut self.db));
        info!("Account with address {:?} created", account.address);
        Ok(account_result)
    }

    pub fn log_user_in(
        &mut self,
        email_address: &str,
        password: String,
    ) -> Result<Session, CambioError> {
        let query = repository::UserClause::EmailAddress(email_address.to_owned());
        let user_option = try!(self.user_repository.read(&query)).pop();
        info!("Logging in {}", email_address);
        if user_option.is_none() {
            info!("User {} does not exist", email_address);
            return Err(CambioError::not_found_search(
                &format!("Could not find account for user {}", email_address),
                "User repository returned None for User",
            ));
        }
        let user = user_option.unwrap();
        if !user.hash_matches_password(&password) {
            info!("Hash does not match password");
            return Err(CambioError::invalid_password());
        }
        let user_id = user.id.unwrap();

        drop(password);

        let mut session = Session::new(email_address, user_id, SESSION_TIME_MILLISECONDS);
        info!("Creating a session");
        let session = try!(self.session_repository.create(&session));
        Ok(session)
    }

    pub fn log_user_out(&mut self, email_address: &str) -> Result<(), CambioError> {
        let query = repository::UserClause::EmailAddress(email_address.to_owned());
        let sessions = try!(self.session_repository.read(&query));
        for mut session in sessions.into_iter() {
            session.session_state = SessionState::Invalidated;
            try!(self.session_repository.update(&session));
        }
        Ok(())
    }
}

const SESSION_TIME_MILLISECONDS: i32 = 1000 * 60 * 15;

const GET_USER_QUERY: &'static str =
    "SELECT id, email_address, password_hash FROM users WHERE email_address = $1";
const GET_USER_QUERY_ID: &'static str =
    "SELECT id, email_address, password_hash FROM users WHERE id = $1";
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

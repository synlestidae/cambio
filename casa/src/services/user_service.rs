use bcrypt::hash;
use api::{RegistrationConfirm, PersonalDetails};
use checkmail;
use db::{CambioError, PostgresHelper};
use domain::{Id, Session, SessionState, User, Registration, Profile, Address, PersonalIdentity, EthAccount, Account, AssetType, AccountRole};
use repository::*;
use postgres::GenericConnection;
use services;
use std::error::Error;

pub struct UserService {
    web3_address: String
}

const BCRYPT_COST: u32 = 8;

impl UserService {
    pub fn new(web3_address: &str) -> Self {
        Self {
            web3_address: web3_address.to_string()
        }
    }

    pub fn confirm_registration<T: GenericConnection>(&self, 
        db: &mut T,
        registration: &Registration,
        personal_details: &PersonalDetails,
        eth_password: &str) -> Result<User, CambioError> {
        // TODO transaction needed here
        println!("Confirming registration");
        
        // Mark the registration as confirmed
        let mut confirmed_registration = registration.clone();
        confirmed_registration.confirm();
        try!(confirmed_registration.update(db));
        println!("Creating registration...");

        // Create and store the user, ready to log in
        let user = try!(self.create_user(
            db,
            &confirmed_registration.email_address, 
            &confirmed_registration.password_hash,
            &personal_details,
            eth_password)
        );

        Ok(user)
    }

    pub fn register_user<T: GenericConnection>(&self,
        db: &mut T,
        email_address: &str,
        password: &str,
        personal_details: &PersonalDetails) -> Result<User, CambioError> {
        // get the BCrypt hash
        let password_hash = try!(hash(password, BCRYPT_COST));
        self.create_user(db, email_address, &password_hash, personal_details, password)
    }

    pub fn create_user<T: GenericConnection>(
        &self,
        db: &mut T,
        email_address: &str,
        password_hash: &str,
        personal_details: &PersonalDetails,
        eth_password: &str) -> Result<User, CambioError> {
        println!("Starting transaction");
        let mut db_tx = try!(db.transaction());
        if !checkmail::validate_email(&email_address.to_owned()) {
            return Err(CambioError::bad_input(
                "Please check that the email entered is valid",
                "Email address is invalid",
            ));
        }
        println!("Getting an option");

        if let Some(_) = try!(email_address.get_option(&mut db_tx)) {
            return Err(CambioError::user_exists());
        }
        let mut user = User {
            id: None,
            email_address: email_address.to_owned(),
            password: None,
            password_hash: Some(password_hash.to_owned()),
            owner_id: None,
        };
        let mut wallet = Account::new_wallet(AssetType::NZD);
        let mut hold = Account::new_hold(AssetType::NZD);

        println!("Creating user!");
        user = try!(user.create(&mut db_tx));
        wallet.owner_user_id = user.owner_id;
        hold.owner_user_id = user.owner_id;

        println!("Creating wallets");
        try!(wallet.create(&mut db_tx));
        try!(hold.create(&mut db_tx));

        println!("Creating eth accounts");
        try!(self.create_eth_accounts(&mut db_tx, email_address, eth_password));
        println!("Eth accounts ready. Creating profile");
        let profile = personal_details.clone().into_profile(user.id.unwrap());
        let new_profile = try!(profile.create(&mut db_tx));
        println!("Profile created");

        println!("Committing!");
        db_tx.set_commit();
        Ok(user)
    }

    fn create_eth_accounts<T: GenericConnection>(&self, 
        db: &mut T,
        email_address: &str, 
        password: &str
    ) -> Result<EthAccount, CambioError> {
        println!("Creating ethereum account for {}", email_address);
        let mut eth_service = services::EthereumService::new(&self.web3_address);
        let account = try!(eth_service.new_account(db, email_address, password));
        println!("Eth account created. Saving...");
        let account_result = try!(account.create(db));
        println!("Account with address {:?} created", account.address);
        Ok(account_result)
    }

    pub fn log_user_in<T: GenericConnection>(&self,
        db: &mut T,
        email_address: &str,
        password: String,
    ) -> Result<Session, CambioError> {
        let user_option = try!(email_address.get_option(db));
        println!("Logging in {}", email_address);
        if user_option.is_none() {
            println!("User {} does not exist", email_address);
            return Err(CambioError::not_found_search(
                &format!("Could not find account for user {}", email_address),
                "User repository returned None for User",
            ));
        }
        let user = user_option.unwrap();
        if !user.hash_matches_password(&password) {
            println!("Hash does not match password");
            return Err(CambioError::invalid_password());
        }
        let user_id = user.id.unwrap();

        drop(password);

        let mut session = Session::new(email_address, user_id, SESSION_TIME_MILLISECONDS);
        println!("Creating a session");
        let session = try!(session.create(db));
        Ok(session)
    }

    pub fn log_user_out<C: GenericConnection>(&mut self, db: &mut C, email_address: &str) -> Result<(), CambioError> {
        const LOG_OUT: &'static str = "
            UPDATE session_info SET 
            session_state = 'invalidated'
            FROM user_session 
            JOIN users ON user_session.user_id = users.id
            WHERE users.email_address = $1";
        try!(db.execute(email_address, &[&email_address]));
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

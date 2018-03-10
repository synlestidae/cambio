use api::{AccountApiTrait, ApiResult, ApiError, ErrorType};
use domain::{Account, Id, Transaction};
use db::{ConnectionSource, PostgresHelper};
use repositories::{AccountRepository, SessionRepository, UserRepository};
use repository;
use services::AccountService;
use repository::RepoRead;

#[derive(Clone)]
pub struct AccountApiImpl<C: PostgresHelper> {
    account_repo: AccountRepository<C>,
    account_service: AccountService<C>,
    session_service: SessionRepository<C>,
    user_repo: UserRepository<C>
}

impl<C: PostgresHelper> AccountApiImpl<C> {
    pub fn new(helper: C) -> Self {
        Self {
            account_repo: AccountRepository::new(helper.clone()),
            account_service: AccountService::new(helper.clone()),
            session_service: SessionRepository::new(helper.clone()),
            user_repo: UserRepository::new(helper)
        }
    }

    fn check_owner(&mut self, owner_id: Id, session_token: &str) -> Result<(), ApiError> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = self.session_service.read(&clause).unwrap().pop().unwrap();
        if session.is_valid() {
            return Err(ApiError::new("You are not logged in.".to_owned(), ErrorType::NotLoggedIn));
        }
        let user = self.user_repo.get_owner(owner_id).unwrap();
        if session.email_address.unwrap() != user.email_address {
            return Err(ApiError::new("Cannot retrieve object.".to_owned(), ErrorType::NotFound));
        }
        Ok(())
    }
}

impl<C: PostgresHelper> AccountApiTrait for AccountApiImpl<C> {
    fn get_accounts(&mut self, email_address: &str, session_token: &str) 
        -> ApiResult<Vec<Account>> {
        let clause = repository::UserClause::EmailAddress(email_address.to_owned());
        unimplemented!()
    }

    fn get_account(&mut self, account_id: &Id, session_token: &str) 
        -> ApiResult<Account> {
        unimplemented!()
    }

    fn get_transactions(&mut self, account_id: &Id, session_token: &str) 
        -> ApiResult<Vec<Transaction>> {
        unimplemented!()
    }

    fn get_transaction(&mut self, account_id: &Id, transaction_id: &Id, session_token: &str) 
        -> ApiResult<Transaction> {
        unimplemented!()
    }
}

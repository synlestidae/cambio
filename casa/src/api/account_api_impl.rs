use api::{AccountApiTrait, ApiResult, ApiError, ErrorType};
use domain::{Account, Id, Transaction};
use db::{ConnectionSource, PostgresHelper, CambioError};
use repositories::{AccountRepository, SessionRepository, UserRepository};
use repository;
use services::AccountService;
use repository::RepoRead;

#[derive(Clone)]
pub struct AccountApiImpl<C: PostgresHelper> {
    account_repo: AccountRepository<C>,
    account_service: AccountService<C>,
    session_repo: SessionRepository<C>,
    user_repo: UserRepository<C>
}

impl<C: PostgresHelper> AccountApiImpl<C> {
    pub fn new(helper: C) -> Self {
        Self {
            account_repo: AccountRepository::new(helper.clone()),
            account_service: AccountService::new(helper.clone()),
            session_repo: SessionRepository::new(helper.clone()),
            user_repo: UserRepository::new(helper)
        }
    }

    fn check_owner(&mut self, owner_id: Id, session_token: &str) -> Result<(), ApiError> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = self.session_repo.read(&clause).unwrap().pop().unwrap();
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
    fn get_accounts(&mut self, session_token: &str) 
        -> ApiResult<Vec<Account>> {
        let clause = repository::UserClause::SessionToken(session_token.to_owned());
        let session = try!(self.session_repo.read(&clause)).pop().unwrap();
        let email_clause = repository::UserClause::EmailAddress(session.email_address.unwrap());
        let accounts = try!(self.account_repo.read(&email_clause));
        let visible_accounts = accounts.into_iter().filter(|a| a.is_user_visible()).collect();
        Ok(visible_accounts)
    }

    fn get_account(&mut self, account_id: Id, session_token: &str) 
        -> ApiResult<Account> {
        let clause = repository::UserClause::Id(account_id);
        let mut accounts = try!(self.account_repo.read(&clause));
        let session = try!(self.session_repo.read(&clause)).pop().unwrap();
        match accounts.pop() {
            Some(account) => {
                let owner_user_id = account.owner_user_id.unwrap();
                let user_clause = repository::UserClause::Id(owner_user_id);
                let account_user = try!(self.user_repo.read(&user_clause)).pop().unwrap();
                if session.email_address != Some(account_user.email_address) {
                    unimplemented!()
                }
                Ok(account)
            }, 
            None => {
                Err(ApiError::not_found("Account"))
            }
        }
    }

    fn get_transactions(&mut self, account_id: Id, session_token: &str) 
        -> ApiResult<Vec<Transaction>> {
        let account = try!(self.get_account(account_id, session_token));
        let statement = try!(self.account_service.get_latest_statement(account_id));
        Ok(statement.transactions)
    }

    fn get_transaction(&mut self, account_id: Id, transaction_id: Id, session_token: &str) 
        -> ApiResult<Transaction> {
        let transactions = try!(self.get_transactions(account_id, session_token));
        let mut txs: Vec<Transaction> = transactions.into_iter().filter(|a| a.id == transaction_id).collect();
        match txs.pop() {
            Some(tx) => Ok(tx),
            None => Err(ApiError::not_found("Transaction"))
        }
    }
}

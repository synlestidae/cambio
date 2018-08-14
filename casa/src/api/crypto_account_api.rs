use postgres::GenericConnection;
use domain::User;
use api::CryptoAccountRequest;
use db::CambioError;
use domain::EthAccount;

pub struct CryptoAccountApi<C: GenericConnection> {
    db: C
}

impl<C: GenericConnection> CryptoAccountApi<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db
        }
    }

    pub fn new_account(&mut self, user: &User, account_request: &CryptoAccountRequest) 
        -> Result<EthAccount, CambioError> {
            unimplemented!()
    }

    pub fn edit_account(&mut self, user: &User, account_request: &CryptoAccountRequest) 
        -> Result<EthAccount, CambioError> {
            unimplemented!()
    }

    pub fn delete_account(&mut self, user: &User, account_request: &CryptoAccountRequest) 
        -> Result<EthAccount, CambioError> {
            unimplemented!()
    }
}

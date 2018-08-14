use postgres::GenericConnection;
use domain::User;
use api::CryptoAccountRequest;
use db::CambioError;
use domain::EthAccount;
use repository::{Readable, Creatable, Updateable};

pub struct CryptoAccountApi<C: GenericConnection> {
    db: C
}

impl<C: GenericConnection> CryptoAccountApi<C> {
    pub fn new(db: C) -> Self {
        Self {
            db: db
        }
    }


    pub fn get_accounts(&mut self, user: &User) -> Result<Vec<EthAccount>, CambioError> {
        Ok(user.owner_id.unwrap().get_vec(&mut self.db)?)
    }

    pub fn new_account(&mut self, user: &User, account_request: &CryptoAccountRequest) 
        -> Result<EthAccount, CambioError> {
            let mut tx = self.db.transaction()?;
            if let Some(..) = account_request.id.unwrap().get_option(&mut tx)? {
                unimplemented!()
            }
            let req = account_request.clone();
            let eth_account = EthAccount {
                id: None,
                owner_id: user.owner_id.unwrap(),
                address: req.address,
                name: req.name
            };
            let created_account = eth_account.create(&mut tx)?;
            tx.commit()?;
            Ok(created_account)
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

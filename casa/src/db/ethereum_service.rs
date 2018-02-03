use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id, EthAccount, EthereumOutboundTransaction};
use web3;
use web3::futures::Future;
use hex;
use web3::types::{H160, H512, Bytes, H256};
use std::str::FromStr;
use db::UserRepository;

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T,
    user_repo: UserRepository<T>,
    web3_address: String
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn new(db_helper: T, web3_address: &str) -> Self {
        let user_repo = UserRepository::new(db_helper.clone());
        Self {
            db_helper: db_helper,
            user_repo: user_repo,
            web3_address: web3_address.to_owned()
        }
    }

    pub fn new_account(&mut self, user_email: &str, account_password: String) -> Result<EthAccount, PostgresHelperError> {
        let web3 = try!(self.get_eth_inst());
        let owner_id = try!(self.user_repo.get_owner_id_by_email_address(user_email));
        match web3.personal().new_account(&account_password).wait() {
            Ok(address) => Ok(EthAccount::new(&address, account_password, owner_id)),
            Err(error) => Err(PostgresHelperError::new(&format!("Failed to create account: {:?}", error)))
        }
    }

    pub fn register_transaction(&mut self, 
        account: &EthAccount, 
        password: String,
        amount_wei: u64,
        destination_address: H256) -> Result<EthereumOutboundTransaction, PostgresHelperError> {
        unimplemented!()
    }

    fn get_eth_inst(&self) -> Result<web3::Web3<web3::transports::http::Http>, PostgresHelperError> {
        // TODO make this use some kind of connection pool if need be
        match web3::transports::Http::new(&self.web3_address) {
            Ok((_eloop, transport)) => Ok(web3::Web3::new(transport)),
            Err(err) => Err(PostgresHelperError::new(&format!("Failed to connect to geth: {:?}", err)))
        }
    }
}

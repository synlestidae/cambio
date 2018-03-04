use chrono::prelude::*;
use services::UserService;
use db::{PostgresHelper, CambioError, ErrorKind, ErrorReccomendation};
use domain::{Order, OrderSettlement, Id, EthAccount, EthereumOutboundTransaction};
use hex;
use repositories;
use repository::*;
use repository;
use std::str::FromStr;
use web3::futures::Future;
use web3::types::{H160, H512, Bytes, H256, U256, TransactionRequest};
use web3;

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T,
    user_repo: repositories::UserRepository<T>,
    web3_address: String
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn new(db_helper: T, web3_address: &str) -> Self {
        let user_repo = repositories::UserRepository::new(db_helper.clone());
        println!("Will connect to {}", web3_address);
        Self {
            db_helper: db_helper,
            user_repo: user_repo,
            web3_address: web3_address.to_owned()
        }
    }

    pub fn new_account(&mut self, user_email: &str, account_password: String) -> Result<EthAccount, CambioError> {
        let (_eloop, web3) = try!(self.get_web3_inst());
        let query = repository::UserClause::EmailAddress(user_email.to_owned());
        let err = CambioError::not_found_search(
            "Cannot create account for unknown user",
            "Failed to find user or owner_id"
        );
        let user_match = try!(self.user_repo.read(&query)).pop();
        let user = try!(user_match.ok_or(err));
        let owner_id = user.owner_id.unwrap();
        let account_result = web3.personal().new_account(&account_password).wait();
        let address = try!(account_result);
        Ok(EthAccount::new(&address, account_password, owner_id))
    }

    pub fn register_transaction(&mut self, 
        account: &EthAccount, 
        password: String,
        amount_wei: u64,
        max_cost_wei: u64,
        destination_address: H160,
        unique_id: &str) -> Result<EthereumOutboundTransaction, CambioError> {

        const BLOCK_CONFIRMATIONS: u64 = 4;

        let (_eloop, web3) = try!(self.get_web3_inst());
        let personal = web3.personal();
        let eth = web3.eth();
        let gas_price_wei = try!(eth.gas_price().wait());
        let block = try!(eth.block_number().wait());
        let confirmations = block.low_u64() + BLOCK_CONFIRMATIONS;
        let gas = U256::from(21000);
        let transaction_req = TransactionRequest {
           from: account.address,
           to: Some(destination_address),
           gas: Some(gas),
           gas_price: Some(gas_price_wei),
           value: Some(U256::from(amount_wei)),
           data: None, 
           nonce: None,
           condition: Some(web3::types::TransactionCondition::Block(confirmations))
        };
        let gas_wei = try!(eth.gas_price().wait());
        if gas_wei > U256::from(max_cost_wei) {
            panic!("Need to put an error here");
        }
        let account_unlocked = try!(personal.unlock_account(account.address, &password, None).wait());
        if !account_unlocked {
            let mut err = CambioError::shouldnt_happen("Failed to get your Ethereum account. Try again.", "Unlocking account failed.");
            err.reccomendation = ErrorReccomendation::TryAgainNow;
            return Err(err);
        }
        let hash = try!(eth.send_transaction(transaction_req).wait());
        let transaction = try!(eth.transaction(web3::types::TransactionId::Hash(hash)).wait());
        if let Some(eth_transaction) = transaction {
            Ok(EthereumOutboundTransaction {
                id: None,
                eth_transaction: eth_transaction,
                unique_id: unique_id.to_string()
            })    
        } else {
            Err(CambioError::not_found_search("Could not find transaction on the block", "eth.transaction returned None"))
        }
    }

    fn get_web3_inst(&self) -> Result<Web3Pair, CambioError> {
        // TODO make this use some kind of connection pool if need be
        let (_eloop, transport) = 
            try!(web3::transports::ipc::Ipc::new(&self.web3_address));
        Ok((_eloop, web3::Web3::new(transport)))
    }
}

pub type Web3Pair = (web3::transports::EventLoopHandle, web3::Web3<web3::transports::ipc::Ipc>);
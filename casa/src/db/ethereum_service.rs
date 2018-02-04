use db::{PostgresHelper, PostgresHelperError};
use chrono::prelude::*;
use domain::{Order, OrderSettlement, Id, EthAccount, EthereumOutboundTransaction};
use web3;
use web3::futures::Future;
use hex;
use web3::types::{H160, H512, Bytes, H256, U256, TransactionRequest};
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
        //println!("Gas price: {}", web3.eth().gas_price().wait().unwrap());
        match web3.personal().new_account(&account_password).wait() {
            Ok(address) => Ok(EthAccount::new(&address, account_password, owner_id)),
            Err(web3::Error::Transport(error_msg)) => {
                Err(PostgresHelperError::new(&format!("Failed to communicate with geth: {:?}", error_msg)))
            },
            Err(error) => {
                Err(PostgresHelperError::new(&format!("Failed to create account: {:?}", error)))
            }
        }
    }

    pub fn register_transaction(&mut self, 
        account: &EthAccount, 
        password: String,
        amount_wei: u64,
        max_cost_wei: u64,
        destination_address: H160,
        unique_id: &str) -> Result<EthereumOutboundTransaction, PostgresHelperError> {
        const BLOCK_CONFIRMATIONS: u64 = 4;

        let web3 = try!(self.get_eth_inst());
        let personal = web3.personal();
        let eth = web3.eth();
        let gas_price_wei = try!(eth.gas_price().wait());
        let block = try!(eth.block_number().wait());
        let confirmations = block.low_u64() + BLOCK_CONFIRMATIONS;
        let transaction_req = TransactionRequest {
           from: account.address,
           to: Some(destination_address),
           gas: Some(gas_price_wei),
           gas_price: Some(gas_price_wei),
           value: Some(U256::from(amount_wei)),
           data: None, 
           nonce: None,
           condition: Some(web3::types::TransactionCondition::Block(confirmations))
        };
        let account_unlocked = try!(personal.unlock_account(account.address, &password, None).wait());
        if !account_unlocked {
            return Err(PostgresHelperError::new("Failed to open account"));
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
            Err(PostgresHelperError::new("Could not find transaction on the block"))
        }
    }

    fn get_eth_inst(&self) -> Result<web3::Web3<web3::transports::ipc::Ipc>, PostgresHelperError> {
        // TODO make this use some kind of connection pool if need be
        match web3::transports::ipc::Ipc::new(&self.web3_address) {
            Ok((_eloop, transport)) => Ok(web3::Web3::new(transport)),
            Err(err) => Err(PostgresHelperError::new(&format!("Failed to connect to geth: {:?}", err)))
        }
    }
}

use chrono::prelude::*;
use services::UserService;
use db::{CambioError, ErrorKind, ErrorReccomendation, PostgresHelper};
use domain::{EthAccount, EthTransferRequest, EthereumOutboundTransaction, Id, Order,
             OrderSettlement};
use hex;
use repositories;
use repository::*;
use repository;
use std::str::FromStr;
use web3::futures::Future;
use web3::types::{Bytes, H160, H256, H512, TransactionRequest, U256};
use web3;

#[derive(Clone)]
pub struct EthereumService<T: PostgresHelper> {
    db_helper: T,
    user_repo: repositories::UserRepository<T>,
    web3_address: String,
}
impl<T: PostgresHelper> EthereumService<T> {
    pub fn new(db_helper: T, web3_address: &str) -> Self {
        let user_repo = repositories::UserRepository::new(db_helper.clone());
        Self {
            db_helper: db_helper,
            user_repo: user_repo,
            web3_address: web3_address.to_owned(),
        }
    }

    pub fn new_account(
        &mut self,
        user_email: &str,
        account_password: String,
    ) -> Result<EthAccount, CambioError> {
        let (_eloop, web3) = try!(self.get_web3_inst());
        let query = repository::UserClause::EmailAddress(user_email.to_owned());
        let err = CambioError::not_found_search(
            "Cannot create account for unknown user",
            "Failed to find user or owner_id",
        );
        let user_match = try!(self.user_repo.read(&query)).pop();
        let user = try!(user_match.ok_or(err));
        let owner_id = user.owner_id.unwrap();
        let account_result = web3.personal().new_account(&account_password).wait();
        let address = try!(account_result);
        Ok(EthAccount::new(&address, account_password, owner_id))
    }

    fn get_request(
        &mut self,
        transfer: &EthTransferRequest,
    ) -> Result<TransactionRequest, CambioError> {
        const BLOCK_CONFIRMATIONS: u64 = 4;
        const GAS_TRANSFER: u64 = 21000;

        let (_eloop, web3) = try!(self.get_web3_inst());
        let personal = web3.personal();
        let eth = web3.eth();

        let gas_price_wei = try!(eth.gas_price().wait());
        let block = try!(eth.block_number().wait());
        let confirmations = block.low_u64() + BLOCK_CONFIRMATIONS;
        let gas_cost: U256 = gas_price_wei.full_mul(U256::from(GAS_TRANSFER)).into();
        if (gas_cost > transfer.max_fee) {
            return Err(CambioError::over_user_limit(
                "Gas price too high for your maximum fee.",
                "Gas price too high: gas_price_wei * 21000 > max_fee",
            ));
        }
        let gas = U256::from(GAS_TRANSFER);
        Ok(TransactionRequest {
            from: transfer.from_address,
            to: Some(transfer.to_address),
            gas: Some(gas),
            gas_price: Some(gas_price_wei),
            value: Some(transfer.value_wei),
            data: None,
            nonce: None,
            condition: Some(web3::types::TransactionCondition::Block(confirmations)),
        })
    }

    pub fn send_transaction(
        &mut self,
        request: TransactionRequest,
    ) -> Result<web3::types::Transaction, CambioError> {
        let (_eloop, web3) = try!(self.get_web3_inst());
        let eth = web3.eth();

        let hash = try!(eth.send_transaction(request).wait());
        let transaction = try!(
            eth.transaction(web3::types::TransactionId::Hash(hash))
                .wait()
        );
        let not_found_err = CambioError::not_found_search(
            "Could not find transaction on the block",
            "eth.transaction returned None",
        );

        match transaction {
            Some(tx) => Ok(tx),
            None => Err(not_found_err),
        }
    }

    pub fn register_transaction(
        &mut self,
        source_account: &EthAccount,
        password: String,
        dst_account: &EthAccount,
        value_wei: U256,
        max_fee: U256,
        unique_id: &str,
    ) -> Result<EthereumOutboundTransaction, CambioError> {
        let (_eloop, web3) = try!(self.get_web3_inst());
        let personal = web3.personal();
        try!(
            personal
                .unlock_account(source_account.address, &password, None)
                .wait()
        );

        let transfer = EthTransferRequest {
            from_address: source_account.address,
            to_address: dst_account.address,
            value_wei: value_wei,
            max_fee,
        };

        let tx_request = try!(self.get_request(&transfer));
        let outbound_transaction = try!(self.send_transaction(tx_request));

        unimplemented!()
    }

    fn get_web3_inst(&self) -> Result<Web3Pair, CambioError> {
        // TODO make this use some kind of connection pool if need be
        let (_eloop, transport) = try!(web3::transports::ipc::Ipc::new(&self.web3_address));
        Ok((_eloop, web3::Web3::new(transport)))
    }
}

pub type Web3Pair = (
    web3::transports::EventLoopHandle,
    web3::Web3<web3::transports::ipc::Ipc>,
);

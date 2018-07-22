use chrono::prelude::*;
use db::{CambioError, ErrorKind, ErrorReccomendation};
use domain::{
    EthAccount, EthTransferRequest, EthereumOutboundTransaction, Id, Order, OrderSettlement,
};
use hex;
use postgres::GenericConnection;
use repository::Readable;
use services::UserService;
use std::str::FromStr;
use web3;
use web3::futures::Future;
use web3::types::{Bytes, H160, H256, H512, TransactionRequest, U256};

pub struct EthereumService {
    web3: web3::Web3<web3::transports::ipc::Ipc>
}

impl EthereumService {
    pub fn new(web3: web3::Web3<web3::transports::ipc::Ipc>) -> Self {
        Self {
            web3: web3
        }
    }

    pub fn new_account<C: GenericConnection>(
        &self,
        db: &mut C,
        user_email: &str,
        account_password: &str,
    ) -> Result<EthAccount, CambioError> {
        let user = try!(Readable::get(user_email, db));
        let owner_id = user.owner_id.unwrap();
        // TODO comment out when web3 decides to work
        let account_result = self.web3.personal().new_account(account_password).wait();
        let address = try!(account_result);
        Ok(EthAccount::new(
            &address,
            account_password.to_owned(),
            owner_id,
        ))
    }

    fn get_request(
        &self,
        transfer: &EthTransferRequest,
    ) -> Result<TransactionRequest, CambioError> {
        const BLOCK_CONFIRMATIONS: u64 = 4;
        const GAS_TRANSFER: u64 = 21000;
        let personal = self.web3.personal();
        let eth = self.web3.eth();

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
        let eth = self.web3.eth();

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
        let personal = self.web3.personal();
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
        let eth_transaction = try!(self.send_transaction(tx_request));
        Ok(EthereumOutboundTransaction {
            id: None,
            eth_transaction: eth_transaction
        })
    }
}

pub type Web3Pair = (
    web3::transports::EventLoopHandle,
    web3::Web3<web3::transports::ipc::Ipc>,
);

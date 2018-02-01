use domain::{Id};
use db::{TryFromRow, TryFromRowError};
use postgres;
use web3::types::{Transaction, U256, H160, H256, Bytes};
use std::str::FromStr;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumOutboundTransaction {
    id: Option<Id>,
    nonce: u64,
    gas_price: u64,
    gas_limit: u64,
    to_address: String,
    from_address: String,
    hash: String,
    value: u64,
    signature: Option<String>,
    transaction_block_id: Option<Id>,
    unique_id: String
}

impl EthereumOutboundTransaction {
    pub fn get_web3_transaction(&self, private_key: &H160) 
        -> Result<EthereumOutboundTransaction, ()> {
        let mut transaction = Transaction {
            nonce: U256::from(self.nonce),
            block_hash: None,
            block_number: None,
            transaction_index: None,
            from: H160::from_str(&self.from_address).unwrap(),
            to: Some(H160::from_str(&self.to_address).unwrap()),
            value: U256::from(self.value),
            gas_price: U256::from(self.gas_price),
            gas: U256::from(self.gas_limit),
            input: Bytes::from(vec![]),
            hash: H256::default()
        };
        let nonce = transaction.nonce.to_vec(); 
        let gas_price = transaction.gas_price.to_vec();
        let gas = transaction.gas.to_vec();
        let to = transaction.to.unwrap().to_vec();
        let value = transaction.value.to_vec();
        let data: Vec<u8> = Vec::new();
        // TODO v r s 
        unimplemented!() 
    }
}

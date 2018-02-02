use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use db::{TryFromRow, TryFromRowError};
use domain::{Id};
use postgres;
use rlp::encode;
use rlp;
use std::str::FromStr;
use web3::api::Eth;
use web3::types::{Transaction, U256, H160, H256, H520, Bytes};
use web3;

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
    pub fn get_web3_transaction<T: web3::Transport>(&self, private_key: &H160, eth: &mut Eth<T>) 
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
        let rlp_transaction = rlp::encode_list::<Vec<u8>, Vec<u8>>(&[
            nonce,
            gas_price,
            gas,
            value,
            data,
            to
        ]);
        let transaction_bytes = rlp_transaction.into_vec();
        let mut sha3 = Sha3::new(Sha3Mode::Keccak256);
        let mut output = Vec::new();
        sha3.input(&transaction_bytes);
        sha3.result(&mut output);
        transaction.hash = H256::from(&output as &[u8]);
        let signature = eth.sign(private_key.clone(), Bytes::from(output));
        
        // now pop the signature onto the vec and send her out!


        //Eth::signkkk

        // TODO v r s 
        unimplemented!() 
    }
}

use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use db::{TryFromRow, TryFromRowError};
use domain::{Id};
use postgres;
use rlp::encode;
use rlp;
use std::str::FromStr;
use web3::futures::Future;
use web3::api::Eth;
use web3::types::{Transaction, U256, H160, H256, H520, H512, Bytes};
use web3;
use secp256k1;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumOutboundTransaction {
    pub id: Option<Id>,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to_address: String,
    pub from_address: String,
    pub hash: String,
    pub value: u64,
    pub signature: Option<String>,
    pub transaction_block_id: Option<Id>,
    pub unique_id: String
}

impl EthereumOutboundTransaction {
    pub fn get_web3_transaction<T: web3::Transport>(&self, private_key: &H512, eth: &mut Eth<T>) 
        -> Result<Vec<u8>, ()> {
        println!("Gett transactin");
        let from = H160::from_str(&self.from_address).unwrap();
        println!("2");
        let to = Some(H160::from_str(&self.to_address).unwrap());
        println!("Really doing it");
        let mut transaction = Transaction {
            nonce: U256::from(self.nonce),
            block_hash: None,
            block_number: None,
            transaction_index: None,
            from: from,
            to: to,
            value: U256::from(self.value),
            gas_price: U256::from(self.gas_price),
            gas: U256::from(self.gas_limit),
            input: Bytes::from(vec![]),
            hash: H256::default()
        };
        println!("make vecs");
        let nonce = transaction.nonce.to_vec(); 
        let gas_price = transaction.gas_price.to_vec();
        let gas = transaction.gas.to_vec();
        let to = transaction.to.unwrap().to_vec();
        let value = transaction.value.to_vec();
        let data: Vec<u8> = Vec::new();
        let rlp_transaction = rlp::encode_list::<Vec<u8>, Vec<u8>>(&[
            nonce.clone(),
            gas_price.clone(),
            gas.clone(),
            to.clone(),
            value.clone(),
            data.clone()
        ]);
        let transaction_bytes = rlp_transaction.into_vec();
        let mut sha3 = Sha3::new(Sha3Mode::Keccak256);
        let mut output = Vec::new();
        sha3.input(&transaction_bytes);
        sha3.result(&mut output);
        transaction.hash = H256::from(&output as &[u8]);
        let mut sig_struct = secp256k1::Secp256k1::new();
        let message = secp256k1::Message::from_slice(&transaction_bytes).unwrap();
        let key = secp256k1::key::SecretKey::from_slice(&sig_struct,
                                                        &private_key.to_vec()).unwrap();
        let signature = sig_struct.sign_schnorr(&message, &key).unwrap().serialize();
        /*let signature = 
            eth.sign(private_key.clone(), Bytes::from(output))
            .wait()
            .unwrap()
            .to_vec();*/
        

        let signed_transaction = rlp::encode_list::<Vec<u8>, Vec<u8>>(&[
            nonce,
            gas_price,
            gas,
            value,
            data,
            to,
            signature
        ]);
        // now pop the signature onto the vec and send her out!
        Ok(signed_transaction.into_vec())
    }
}

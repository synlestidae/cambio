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
use web3::types::{Transaction, U256, H160, H256, H520, H512, Bytes, H64};
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
    pub fn get_web3_transaction<T: web3::Transport>(&self, private_key: &H256, eth: &mut Eth<T>) 
        -> Result<Vec<u8>, ()> {
        println!("Gett transactin");
        let from = H160::from_str(&self.from_address).unwrap();
        println!("2");
        let to = Some(H160::from_str(&self.to_address).unwrap());
        println!("Really doing it");
        let nonce = H256::from(self.nonce).to_vec();//transaction.nonce.to_vec(); 
        let gas_price = U256::from(self.gas_price).to_vec();
        let gas = U256::from(self.gas_limit).to_vec();
        let to = H160::from_str(&self.to_address).unwrap().to_vec();
        let value = U256::from(self.value).to_vec();
        let data: Vec<u8> = Vec::new();
        println!("Nonce {}", nonce.len());
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
        output.resize(32, 0);
        sha3.input(&transaction_bytes);
        sha3.result(&mut output);
        println!("output yo {}", output.len());
        let transaction_hash = H256::from(&output as &[u8]);
        let mut sig_struct = secp256k1::Secp256k1::new();
        println!("transacco {}", transaction_hash.len());
        let message = secp256k1::Message::from_slice(&transaction_hash).unwrap();
        let key = secp256k1::key::SecretKey::from_slice(&sig_struct,
                                                        &private_key.to_vec()).unwrap();
        let signature = sig_struct.sign_schnorr(&message, &key).unwrap().serialize();
        
        println!("NONCE {:?}", nonce);

        let signed_transaction = rlp::encode_list::<Vec<u8>, Vec<u8>>(&[
            nonce,
            gas_price,
            gas,
            value,
            data,
            to,
            signature
        ]);

        println!("POP {:?}", signed_transaction);

        // now pop the signature onto the vec and send her out!
        Ok(signed_transaction.into_vec())
    }
}

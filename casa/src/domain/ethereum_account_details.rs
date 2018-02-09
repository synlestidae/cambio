use base64::{encode, decode};
use crypto::digest::Digest;
use crypto;
use db::{TryFromRow, TryFromRowError};
use domain::{Id, DecryptError};
use openssl::aes;
use openssl::symm;
use postgres;
use rand::{OsRng, Rng};
use rand;
use std::iter;
use std;
use bcrypt::hash;
use web3::types::{H160, U256, Transaction};

const BCRYPT_COST: u32 = 8;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthAccount {
    pub address: H160,
    pub password_hash_bcrypt: String,
    pub owner_id: Id, 
}

impl EthAccount {
    pub fn new(address: &H160, password: String, owner_id: Id) -> Self {
        let bcrypted_password = hash(&password, BCRYPT_COST).unwrap();
        drop(password);
        Self {
            address: *address,
            password_hash_bcrypt: bcrypted_password,
            owner_id: owner_id
        }
    }
}

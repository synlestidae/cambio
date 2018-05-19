use base64::{decode, encode};
use crypto::digest::Digest;
use crypto;
use db::{TryFromRow, TryFromRowError};
use domain::{DecryptError, Id};
use openssl::aes;
use openssl::symm;
use postgres;
use rand::{OsRng, Rng};
use rand;
use std::iter;
use std;
use bcrypt::hash;
use web3::types::{H160, Transaction, U256};

const BCRYPT_COST: u32 = 8;

#[derive(Debug, Clone)]
pub struct EthAccount {
    pub id: Option<Id>,
    pub address: H160,
    pub password_hash_bcrypt: String,
    pub owner_id: Id,
}

impl EthAccount {
    pub fn new(address: &H160, password: String, owner_id: Id) -> Self {
        let bcrypted_password = hash(&password, BCRYPT_COST).unwrap();
        drop(password);
        Self {
            id: None,
            address: *address,
            password_hash_bcrypt: bcrypted_password,
            owner_id: owner_id,
        }
    }
}

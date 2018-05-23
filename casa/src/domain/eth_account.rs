use base64::{decode, encode};
use bcrypt::hash;
use crypto;
use crypto::digest::Digest;
use db::{TryFromRow, TryFromRowError};
use domain::{DecryptError, Id, OwnerId};
use openssl::aes;
use openssl::symm;
use postgres;
use rand;
use rand::{OsRng, Rng};
use std;
use std::iter;
use web3::types::{H160, Transaction, U256};

const BCRYPT_COST: u32 = 8;

#[derive(Debug, Clone)]
pub struct EthAccount {
    pub id: Option<Id>,
    pub address: H160,
    pub password_hash_bcrypt: String,
    pub owner_id: OwnerId,
}

impl EthAccount {
    pub fn new(address: &H160, password: String, owner_id: OwnerId) -> Self {
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

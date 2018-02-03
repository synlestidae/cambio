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
use web3::types::H160;

const BCRYPT_COST: u32 = 8;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthAccount {
    address: H160,
    password_hash_bcrypt: String,
    owner_id: i32
}

impl EthAccount {
    pub fn new(address: &H160, password: String, owner_id: i32) -> Self {
        let bcrypted_password = hash(&password, BCRYPT_COST).unwrap();
        drop(password);
        Self {
            address: *address,
            password_hash_bcrypt: bcrypted_password,
            owner_id: owner_id
        }
    }
}

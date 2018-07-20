use base64::{decode, encode};
use bcrypt::hash;
use crypto;
use crypto::digest::Digest;
use db::{TryFromRow, TryFromRowError};
use domain::{DecryptError, EthAccountId, Id, OwnerId};
use openssl::aes;
use openssl::symm;
use postgres;
use postgres::rows::Row;
use rand;
use rand::{OsRng, Rng};
use std;
use std::iter;
use web3::types::{H160, Transaction, U256};

const BCRYPT_COST: u32 = 8;

#[derive(Debug, Clone)]
pub struct EthAccount {
    pub id: Option<EthAccountId>,
    pub address: H160,
    pub password_hash_bcrypt: String,
    pub owner_id: OwnerId,
}

#[derive(TryFromRow)]
pub struct EthAccountRow {
    pub id: Option<EthAccountId>,
    pub address: Vec<u8>,
    pub password_hash_bcrypt: String,
    pub owner_id: OwnerId,
}

impl TryFromRow for EthAccount {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let eth: EthAccountRow = try!(EthAccountRow::try_from_row(row));
        let mut bytes: [u8; 20] = [0; 20];
        bytes.copy_from_slice(&eth.address);
        Ok(EthAccount {
            id: eth.id,
            address: H160(bytes),
            password_hash_bcrypt: eth.password_hash_bcrypt,
            owner_id: eth.owner_id,
        })
    }
}

impl EthAccount {
    pub fn new(address: &H160, password: String, owner_id: OwnerId) -> Self {
        let bcrypted_password = hash(&password, BCRYPT_COST).unwrap();
        Self {
            id: None,
            address: *address,
            password_hash_bcrypt: bcrypted_password,
            owner_id: owner_id,
        }
    }
}

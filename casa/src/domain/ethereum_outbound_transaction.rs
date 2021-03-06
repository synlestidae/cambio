use crypto::digest::Digest;
use crypto::sha3::{Sha3, Sha3Mode};
use db::{TryFromRow, TryFromRowError};
use domain::Id;
use postgres;
use rlp;
use rlp::encode;
use secp256k1;
use std::str::FromStr;
use web3;
use web3::api::Eth;
use web3::futures::Future;
use web3::types::{Bytes, H160, H256, H512, H520, H64, Transaction, U256};

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumOutboundTransaction {
    pub id: Option<Id>,
    pub eth_transaction: Transaction,
}

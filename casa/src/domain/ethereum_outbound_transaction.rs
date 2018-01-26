use domain::{Id};
use db::{TryFromRow, TryFromRowError};
use postgres;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumOutboundTransaction {
    id: Option<Id>,
    nonce: String,
    gas_price: u64,
    gas_limit: u64,
    to_address: String,
    from_address: String,
    hash: String,
    value: u64,
    signature: String,
    transaction_block_id: Option<Id>
}

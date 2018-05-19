use chrono::prelude::*;
use db::{TryFromRow, TryFromRowError};
use domain::Id;
use postgres;

#[Derive(Debug, Clone, TryFromRow)]
pub struct EthereumBlock {
    id: Option<Id>,
    time: DateTime<Utc>,
    block: u64,
    block_hash: String,
}

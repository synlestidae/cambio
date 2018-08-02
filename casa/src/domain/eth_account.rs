use domain::EthAccountId;
use domain::ByteAddress;
use domain::OwnerId;
use postgres;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

const BCRYPT_COST: u32 = 8;

#[derive(Debug, Clone, TryFromRow)]
pub struct EthAccount {
    pub id: Option<EthAccountId>,
    pub address: ByteAddress,
    pub name: String,
    pub owner_id: OwnerId,
}

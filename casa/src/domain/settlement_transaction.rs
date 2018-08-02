use chrono::prelude::*;
use domain::OrderSettlementId;
use domain::BigInteger;
use domain::ByteAddress;
use web3::types::H160;
use postgres;
use db::TryFromRow;
use db::TryFromRowError;

#[derive(TryFromRow)]
pub struct SettlementTransaction {
    pub settlement_id: OrderSettlementId,
    pub from_address: ByteAddress,
    pub to_address: ByteAddress,
    pub amount_wei: BigInteger,
    pub blockchain_due_datetime: DateTime<Utc>
}

use domain::*;
use chrono::prelude::*;
use web3::types::U256;

#[derive(Serialize)]
pub struct UserSettlement {
    pub source_order: Order,
    pub settlement_status: SettlementStatus,
    pub from_address: ByteAddress,
    pub to_address: ByteAddress,
    pub value: U256,
    pub due_on_blockchain_at: DateTime<Utc>
}

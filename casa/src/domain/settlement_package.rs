use domain::*;
use web3::types::*;
use chrono::Duration;
use postgres::GenericConnection;
use postgres::Error;
use db::CambioError;
use repository::Updateable;

pub struct SettlementPackage {
    pub settlement: OrderSettlement,
    pub original_order: Order,
    pub settling_order: Order,
    pub criteria: SettlementCriteria,
    pub eth_transfer: EthTransfer,
    pub settlement_addr: H160,
    pub criteria_addr: H160
}

impl SettlementPackage {
    pub fn can_proceed(&self) -> bool {
        self.settlement.can_proceed()
    }

    pub fn completes_settlement(&self) -> bool {
        let (expected_from, expected_to) = if self.settlement.settles_buy {
            // if it settles a buy, a customer specified that the order amount goes into
            // criteria_addr
            (self.settlement_addr, self.criteria_addr)
        } else {
            (self.criteria_addr, self.settlement_addr)
        };
        let expected_value = self.original_order.amount_crypto.clone().into();
        return 
            self.eth_transfer.from == expected_from && 
            self.eth_transfer.to == expected_to && 
            self.eth_transfer.value == expected_value;

    }

    // Returns true if the Ethereum transfer was made after settlement but before the due time
    pub fn is_on_time(&self) -> bool {
        let start_time = self.settlement.started_at.timestamp() as u64;
        let end_time = (self.settlement.started_at + Duration::minutes(self.criteria.time_limit_minutes as i64)).timestamp() as u64;

        let eth_timestamp = self.eth_transfer.timestamp.low_u64();

        start_time <= eth_timestamp && eth_timestamp <= end_time
    }

    pub fn mark_settled(&mut self) {
        self.original_order.mark_settled();
        self.settling_order.mark_settled();
        self.settlement.mark_settled();
    }

    pub fn update_fields<C: GenericConnection>(&mut self, db: &mut C) -> Result<(), CambioError> {
        self.original_order.update(db)?;
        self.settling_order.update(db)?;
        self.settlement.update(db)?;
        Ok(())
    }
}

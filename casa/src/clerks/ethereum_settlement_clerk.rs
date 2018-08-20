use postgres::GenericConnection;
use event::*;
use domain::*;
use clerks::eth_transfer::EthTransfer;
use db::CambioError;
use repository::*;
use chrono::Duration;

use web3::types::*;
use postgres::Error as PostgresError;

pub struct EthereumSettlementClerk<C: GenericConnection> {
    bus: Bus,
    db: C
}

impl<C: GenericConnection> EthereumSettlementClerk<C> {
    pub fn handle_eth_transfer(&mut self, eth_transfer: EthTransfer) -> Result<(), CambioError> {
        let addr: ByteAddress = eth_transfer.from.clone().into();
        let settlements: Vec<OrderSettlement> = addr.get_vec(&mut self.db)?;

        for settlement in settlements.into_iter() {
            let mut tx = self.db.transaction()?;
            let criteria: SettlementCriteria = settlement.order_with_criteria().get(&mut tx)?;
            let eth_account = criteria
                .eth_account()
                .ok_or(CambioError::shouldnt_happen(
                        "Could not locate the Ethereum account for the settlement",
                        "Criteria missing reference to Ethereum account"
                ))?
                .get(&mut tx)?;


            // TODO turn this into some kind of builder

            let package = SettlementPackage {
                settlement_addr: settlement.eth_account.get(&mut tx)?.address.into(),
                original_order: settlement.original_order.get(&mut tx)?,
                settling_order: settlement.settling_order.get(&mut tx)?,
                settlement: settlement,
                criteria: criteria,
                eth_transfer: eth_transfer.clone(),
                criteria_addr: eth_account.address.into()
            };

            self.handle_order_settlement(&mut tx, package);
            tx.set_commit();
        }

        Ok(())
    }

    fn handle_order_settlement<D: GenericConnection>(&self, db: &mut D, mut package: SettlementPackage) {
        if !package.can_proceed() {
            warn!("Settlement {:?} cannot proceed", package.settlement.id);
        }
        if !package.completes_settlement() {
            warn!("Settlement does not fulfill order");
        }
        if !package.is_on_time() {
            warn!("Settlement was not on time");
        }
        package.mark_settled();
        match package.update(db) {
            Ok(_) => {},
            Err(err) => {
                error!("Database error while updating settlement: {}.", err);
                error!("Details of update error: {:?}.", err);
                // Settlement will just have to be picked up by routine settlement checker
            }
        }
    }
}

struct SettlementPackage {
    settlement: OrderSettlement,
    original_order: Order,
    settling_order: Order,
    criteria: SettlementCriteria,
    eth_transfer: EthTransfer,
    settlement_addr: H160,
    criteria_addr: H160
}

impl SettlementPackage {
    fn can_proceed(&self) -> bool {
        self.settlement.can_proceed()
    }

    fn completes_settlement(&self) -> bool {
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
    fn is_on_time(&self) -> bool {
        let start_time = self.settlement.started_at.timestamp() as u64;
        let end_time = (self.settlement.started_at + Duration::minutes(self.criteria.time_limit_minutes as i64)).timestamp() as u64;

        let eth_timestamp = self.eth_transfer.timestamp.low_u64();

        start_time <= eth_timestamp && eth_timestamp <= end_time
    }

    fn mark_settled(&mut self) {
        self.original_order.mark_settled();
        self.settling_order.mark_settled();
        self.settlement.mark_settled();
    }

    fn update<C: GenericConnection>(&mut self, db: &mut C) -> Result<(), CambioError> {
        self.original_order = self.original_order.update(db)?;
        self.settling_order = self.settling_order.update(db)?;
        self.settlement = self.settlement.update(db)?;
        Ok(())
    }
}

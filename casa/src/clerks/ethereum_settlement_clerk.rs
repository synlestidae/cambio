use postgres::GenericConnection;
use event::*;
use domain::*;
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
            warn!("Settlement  cannot proceed");
        }
        if !package.completes_settlement() {
            warn!("Settlement does not fulfill order");
        }
        if !package.is_on_time() {
            warn!("Settlement was not on time");
        }
        package.mark_settled();
        match package.update_fields(db) {
            Ok(_) => {},
            Err(err) => {
                error!("Database error while updating settlement: {}.", err);
                error!("Details of update error: {:?}.", err);
                // Settlement will just have to be picked up by routine settlement checker
            }
        }
    }
}


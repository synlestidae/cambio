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
        info!("Transfer of {} from addr {} to {} confirmed", eth_transfer.value, 
              eth_transfer.from, 
              eth_transfer.to);
        let addr: ByteAddress = eth_transfer.from.clone().into();
        info!("Checking settlements related to this transfer");
        let settlements: Vec<OrderSettlement> = addr.get_vec(&mut self.db)?;
        info!("Got {} settlements", settlements.len());
        for settlement in settlements.into_iter() {
            info!("Beginning settlement transaction");
            let mut tx = self.db.transaction()?;
            let criteria: SettlementCriteria = settlement.order_with_criteria().get(&mut tx)?;
            let eth_account = criteria
                .eth_account()
                .ok_or(CambioError::shouldnt_happen(
                        "Could not locate the Ethereum account for the settlement",
                        "Criteria missing reference to Ethereum account"
                ))?
                .get(&mut tx)?;


            info!("Packaging settlement parameters");
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

            if package.addresses_match() {
                info!("Addresses match. Handling settlement now.");
                self.handle_order_settlement(&mut tx, package);
            } else {
                info!("Addresses don't match on transfer");
            }

            tx.set_commit();
        }

        Ok(())
    }

    fn handle_order_settlement<D: GenericConnection>(&self, db: &mut D, mut package: SettlementPackage) {
        if !package.can_proceed() {
            warn!("Settlement cannot proceed. It may already be settled.");
            return;
        }
        if !package.completes_settlement() {
            warn!("THe Ethereum transfer does not fulfill the settlement");
            return;
        }
        if !package.is_on_time() {
            warn!("Settlement was late. This settlement will be marked as failed.");
            return;
        }

        // Danger area

        package.mark_settled();
        match package.update_fields(db) {
            Ok(_) => {},
            Err(err) => {
                error!("Database error while updating settlement: {}.", err);
                error!("Details of update error: {:?}.", err);
                // Settlement will just have to be picked up by routine settlement checker
            }
        }

        // TODO transfer funds from A to B
    }
}


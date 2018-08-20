use postgres::GenericConnection;
use event::*;
use domain::*;
use clerks::eth_transfer::EthTransfer;
use db::CambioError;
use repository::Readable;
use web3::types::*;

pub struct EthereumSettlementClerk<C: GenericConnection> {
    bus: Bus,
    db: C
}

impl<C: GenericConnection> EthereumSettlementClerk<C> {
    pub fn handle_eth_transfer(&mut self, eth_transfer: EthTransfer) -> Result<(), CambioError> {
        let addr: ByteAddress = eth_transfer.from.clone().into();
        let settlements: Vec<OrderSettlement> = addr.get_vec(&mut self.db)?;

        for settlement in settlements.into_iter() {
            let buy: Order = settlement.buying_fiat_id.get(&mut self.db)?;
            let sell: Order = settlement.buying_crypto_id.get(&mut self.db)?;
            let buy_criteria: SettlementCriteria = buy.id.unwrap().get(&mut self.db)?;
            let sell_criteria: SettlementCriteria = sell.id.unwrap().get(&mut self.db)?;
            let (from, to) = match (buy_criteria.from_account, sell_criteria.to_account) {
                (Some(from_id), Some(to_id)) => (from_id.get(&mut self.db)?, to_id.get(&mut self.db)?),
                _ => {
                    unimplemented!() // this would represent an illegal state
                }
            };
            let from_address: H160 = from.address.clone().into();
            let to_address: H160 = to.address.clone().into();
            if (to_address == eth_transfer.from && to_address == eth_transfer.to) {
                // it's a match
                self.handle_order_settlement(settlement, buy, sell, &eth_transfer);
            }
        }

        Ok(())
    }

    fn handle_order_settlement(&mut self, settlement: OrderSettlement, buy_order: Order, sell_order: Order, eth_transfer: &EthTransfer) {
        // check whether already settled
        // check whether transfer matches order
        // check timestamp of transfer
    }
}

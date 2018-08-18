use domain::*;
use event::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Event {
    Order(OrderEvent),
    Settlement(SettlementEvent),
    Blockchain(BlockchainEvent),
}

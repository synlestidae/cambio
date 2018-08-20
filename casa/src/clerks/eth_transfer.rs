use event::EventKey;
use colectivo::MessageKey;
use chrono::prelude::*;
use web3::types::*;
use repository::Readable;

#[derive(Clone, Serialize, Deserialize)]
pub struct EthTransfer {
    pub block_number: U256,
    pub hash: H256,
    pub from: H160,
    pub to: H160,
    pub value: U256,
    pub timestamp: U256
}

impl EthTransfer {
    pub fn from<T>(tx: &Transaction, block: &Block<T>) -> Option<Self> {
        match (tx.block_number, tx.to) {
            (Some(b), Some(to)) => {
                Some(Self {
                    block_number: b.clone(),
                    hash: tx.hash.clone(),
                    from: tx.from.clone(),
                    to: to.clone(),
                    value: tx.value.clone(),
                    timestamp: block.timestamp.clone()
                })
            },
            _ => None
        }
    }
}

impl EventKey for EthTransfer {
    fn key(&self) -> MessageKey {
        MessageKey(self.hash.0.to_vec())
    }
}


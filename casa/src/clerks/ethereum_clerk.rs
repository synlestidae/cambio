use event::Bus;
use event::EventHandler;
use std::error::Error;
use web3::api::Web3;
use web3::Transport;
use web3::transports::EventLoopHandle;
use web3::helpers::CallResult;
use web3::api::BaseFilter;
use web3::api::Eth;
use domain::ByteAddress;
use futures::Future;
use futures::stream::StreamFuture;
use futures::future::{Empty, empty};
use std::time;
use futures::Stream;
use web3::types::*;

pub struct EthereumClerk<T: Transport> {
    bus: Bus,
    web3: Web3<T>,
    handle: EventLoopHandle,
    filter_count: u64
}

impl<T: Transport> EthereumClerk<T> {
    fn subscribe_address(&mut self, address: ByteAddress) {
        let bus_copy = self.bus.clone();
        let eth = self.web3.eth();
        let filter = FilterBuilder::default()
            .address(vec![address.into()])
            .build();

        self.web3
            .eth_filter()
            .create_logs_filter(filter)
            .map(move |event| Self::handle_address_event(bus_copy, eth, event)); 
    }

    fn handle_address_event(bus: Bus, eth: Eth<T>, filter: BaseFilter<T, Log>) -> StreamFuture<T, EthTransfer> {
        let bus_copy = bus.clone();

        filter
            .stream(time::Duration::from_secs(0))
            .filter_map(|log| log.transaction_hash)
            .map(move |tx_hash| {
                eth.transaction(TransactionId::Hash(tx_hash))
            })
            .map(|r| r.map(|et| Self::to_eth_transfer(et)))
            .map(|t| t.map(|transfer| Self::confirm_transfer(bus_copy, eth, transfer)))
            .into_future()
    }

    fn to_eth_transfer(tx_option: Option<Transaction>) -> Option<EthTransfer> {
        if let Some(tx) = tx_option {
            match (tx.block_number, tx.to) {
                (Some(block), Some(to)) => {
                    Some(EthTransfer {
                        block_number: block,
                        tx_hash: tx.hash,
                        from: tx.from,
                        to: to,
                        value: tx.value
                    })
                }
                _ => None
            }
        } else {
            None
        }
    }

    fn confirm_transfer<E, F>(bus: Bus, eth: Eth<T>, transfer: Option<EthTransfer>) -> Empty<E, F> {
        empty()
    }
}


#[derive(Serialize, Deserialize)]
struct EthTransfer {
    block_number: U256,
    tx_hash: H256,
    from: H160,
    to: H160,
    value: U256
}

fn id<T>(x: T) -> T {
    x
}

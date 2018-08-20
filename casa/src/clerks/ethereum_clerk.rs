use clerks::eth_transfer::EthTransfer;
use colectivo::MessageKey;
use domain::ByteAddress;
use event::*;
use futures::Future;
use futures::Stream;
use futures::future::{Empty, empty};
use futures::stream::StreamFuture;
use std::error::Error;
use std::time;
use web3::Transport;
use web3::api::BaseFilter;
use web3::api::Eth;
use web3::api::Web3;
use web3::helpers::CallResult;
use web3::transports::EventLoopHandle;
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
        let filter = FilterBuilder::default()
            .address(vec![address.into()])
            .build();

        let filter_result = self.web3
            .eth_filter()
            .create_logs_filter(filter)
            .wait();

        Self::handle_subscribe_address_logs(bus_copy, 
            self.web3.eth(), 
            filter_result.unwrap().logs().wait().unwrap());
    }

    fn handle_subscribe_address_logs(bus: Bus, eth: Eth<T>, logs: Vec<Log>) {
        let current_block = eth.block_number().wait().unwrap();
        for log in logs.into_iter() {
            let (tx_id, block) = match (log.block_number, log.transaction_index, log.transaction_hash, log.block_number) {
                (_, _, Some(hash), Some(bn)) => (TransactionId::Hash(hash), bn.low_u64()),
                _ => continue
            };
            let block = eth.block(BlockId::Number(BlockNumber::Number(block))).wait().unwrap();
            let transaction = eth.transaction(tx_id).wait().unwrap().unwrap();
            let transfer = EthTransfer::from(&transaction, &block).unwrap();
            let block_number = transfer.block_number; 
            if (current_block - block_number).low_u64() >= 11 {
                bus.send(&transfer, &EthereumEventType::TransferConfirmed); 
            } else {
                bus.send(&transfer, &EthereumEventType::WaitTransferConfirmation); 
            }
        }
    }

    fn handle_transfer_confirmed(transfer: EthTransfer) {
         
    }

    fn wait_transfer_confirmation(transfer: EthTransfer) {

    }

    fn confirm_transfer<E, F>(bus: Bus, eth: Eth<T>, transfer: Option<EthTransfer>) -> Empty<E, F> {
        empty()
    }
}


fn id<T>(x: T) -> T {
    x
}

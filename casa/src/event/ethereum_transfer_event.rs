#[derive(Debug, Serialize, Deserialize)]
pub enum EthereumTransferEvent {
    TransferConfirmed,
    TransferUnconfirmed
}

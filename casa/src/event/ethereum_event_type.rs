#[derive(Debug, Serialize, Deserialize)]
pub enum EthereumEventType {
    SubscribeAddress,
    WaitTransferConfirmation,
    TransferConfirmed
}

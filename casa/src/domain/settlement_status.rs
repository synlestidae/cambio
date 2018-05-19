#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize)]
#[postgres(name = "settlement_status")]
pub enum SettlementStatus {
    #[postgres(name = "settling")] Settling,
    #[postgres(name = "waiting_eth_credentials")] WaitingEthCredentials,
    #[postgres(name = "waiting_eth")] WaitingEth,
    #[postgres(name = "settled")] Settled,
    #[postgres(name = "cancelled")] Cancelled,
    #[postgres(name = "invalid")] Invalid,
    #[postgres(name = "eth_failed")] EthFailed,
}

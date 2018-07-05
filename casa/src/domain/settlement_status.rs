#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize)]
#[postgres(name = "settlement_status")]
pub enum SettlementStatus {
    #[postgres(name = "waiting_eth_credentials")]
    WaitingEthCredentials,
    #[postgres(name = "waiting_eth")]
    WaitingEth,
    #[postgres(name = "settled")]
    Settled,
    #[postgres(name = "cancelled")]
    Cancelled,
    #[postgres(name = "invalid")]
    Invalid,
    #[postgres(name = "eth_failed")]
    EthFailed,
}

impl SettlementStatus {
    pub fn is_settling(&self) -> bool {
        match self {
            SettlementStatus::WaitingEthCredentials => true,
            SettlementStatus::WaitingEth => true,
            _ => false
        }
    }
}

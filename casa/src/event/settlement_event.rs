
#[derive(Debug, Serialize, Deserialize)]
pub enum SettlementEvent {
    SettlementStarted,
    SettlementCompleted,
    SettlementFailed
}

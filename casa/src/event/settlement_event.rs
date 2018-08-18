
#[derive(Debug, Serialize, Deserialize)]
pub struct SettlementEvent {
    pub settlement: OrderSettlement,
    pub event_type: SettlementEventType
}

pub enum SettlementEventType {
    SettlementStarted,
    SettlementCompleted,
    SettlementFailed
}

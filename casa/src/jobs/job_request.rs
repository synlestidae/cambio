use domain::OrderSettlementId;

pub enum JobRequest {
    BeginSettlement(OrderSettlementId, String)
}

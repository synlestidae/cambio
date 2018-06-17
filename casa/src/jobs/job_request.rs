use domain::OrderSettlementId;
use std::marker::Sync;

pub enum JobRequest {
    BeginSettlement(OrderSettlementId, String)
}

unsafe impl Sync for JobRequest {
}

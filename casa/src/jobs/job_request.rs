use domain::OrderSettlementId;
use std::marker::Sync;

#[derive(Debug)]
pub enum JobRequest {
    BeginSettlement(OrderSettlementId, String),
}

unsafe impl Sync for JobRequest {}

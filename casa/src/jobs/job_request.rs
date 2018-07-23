use domain::OrderSettlementId;
use jobs::EmailRequest;
use std::marker::Sync;

#[derive(Debug)]
pub enum JobRequest {
    BeginSettlement(OrderSettlementId, String),
    SendEmail(EmailRequest)
}

unsafe impl Sync for JobRequest {}

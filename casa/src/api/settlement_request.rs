use api;
use domain;

#[derive(Debug)]
pub enum SettlementRequest {
    PostSettlementEthAuth(domain::OrderId, api::SettlementEthCredentials),
    GetSettlementStatus(domain::OrderId),
}

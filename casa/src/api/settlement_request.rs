use api;

#[derive(Debug)]
pub enum SettlementRequest {
    PostSettlementEthAuth(api::SettlementEthCredentials),
    GetSettlementStatus
}

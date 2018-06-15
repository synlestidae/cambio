use api;

pub enum SettlementRequest {
    PostSettlementEthAuth(api::SettlementEthCredentials),
    GetSettlementStatus
}

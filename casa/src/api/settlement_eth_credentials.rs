use domain::{OrderId, OrderSettlementId};

#[derive(Debug, Deserialize, Clone)]
pub struct SettlementEthCredentials {
    pub password: String,
    pub unique_id: String,
}

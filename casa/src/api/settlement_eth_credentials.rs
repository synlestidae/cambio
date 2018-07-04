use domain::{OrderId, OrderSettlementId};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SettlementEthCredentials {
    pub password: String,
    pub unique_id: String,
}

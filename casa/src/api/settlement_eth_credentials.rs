use domain::{OrderId, OrderSettlementId};

#[derive(Deserialize, Clone)]
pub struct SettlementEthCredentials {
    pub password: String,
    pub settlement_id: OrderSettlementId,
    pub order_id: OrderId,
    pub unique_id: String,
}

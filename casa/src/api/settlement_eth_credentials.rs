use domain::Id;

#[derive(Deserialize, Clone)]
pub struct SettlementEthCredentials {
    pub password: String,
    pub settlement_id: Id,
    pub order_id: Id,
}

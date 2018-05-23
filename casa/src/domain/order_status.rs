#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize, ToSql, FromSql)]
#[postgres(name = "order_status")]
pub enum OrderStatus {
    #[postgres(name = "active")]
    Active,
    #[postgres(name = "settling")]
    Settling,
    #[postgres(name = "settled")]
    Settled,
    #[postgres(name = "settlement_failed")]
    SettlementFailed,
    #[postgres(name = "user_cancelled")]
    UserCancelled,
    #[postgres(name = "admin_cancelled")]
    AdminCancelled,
    #[postgres(name = "expired")]
    Expired,
    #[postgres(name = "deleted")]
    Deleted,
}

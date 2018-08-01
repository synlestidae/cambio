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

use std::fmt;

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            &OrderStatus::Active => "active",
            &OrderStatus::Settling => "settling",
            &OrderStatus::Settled => "settled",
            &OrderStatus::SettlementFailed => "settlement_failed",
            &OrderStatus::UserCancelled => "user_cancelled",
            &OrderStatus::AdminCancelled => "admin_cancelled",
            &OrderStatus::Expired => "expired",
            &OrderStatus::Deleted => "deleted",
        };
        write!(f, "{}", val)
    }
}

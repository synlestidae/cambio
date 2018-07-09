#[postgres(name = "payment_status_type")]
pub enum PaymentStatus {
    #[postgres(name = "started_by_user")]
    StartedByUser,
    #[postgres(name = "started_with_poli")]
    StartedWithPoli,
    #[postgres(name = "cancelled")]
    Cancelled,
    #[postgres(name = "failed")]
    Failed,
    #[postgres(name = "unknown")]
    Unknown,
    #[postgres(name = "completed")]
    Completed
}

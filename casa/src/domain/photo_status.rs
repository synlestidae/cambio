#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "photo_status_type")]
pub enum PhotoStatus {
    #[postgres(name = "approved")] Approved,
    #[postgres(name = "denied")] Denied,
    #[postgres(name = "unclear")] Unclear,
    #[postgres(name = "waiting_approval")] WaitingApproval,
}

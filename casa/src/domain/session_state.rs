#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "session_state_type")]
pub enum SessionState {
    #[postgres(name = "valid")]
    Valid,
    #[postgres(name = "invalidated")]
    Invalidated,
}

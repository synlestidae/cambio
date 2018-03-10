#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ErrorReccomendation {
    TryAgainNow,
    TryAgainLater,
    ContactProgrammer,
    ContactSupport,
    CheckInput,
    CheckState,
    Nothing,
    Continue
}

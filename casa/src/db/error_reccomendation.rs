#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

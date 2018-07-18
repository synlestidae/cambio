#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum TransactionStatusCode {
    Initiated,
    FinancialInstitutionSelected,
    EulaAccepted,
    InProcess,
    Unknown,
    Completed,
    Cancelled,
    Failed,
    ReceiptUnverified,
    TimedOut
}

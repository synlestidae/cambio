#[derive(Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionStatusCode {
    Initiated,
    FinancialInstitutionSelected
}

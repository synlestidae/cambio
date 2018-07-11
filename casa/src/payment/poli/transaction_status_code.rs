#[derive(Eq, PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum TransactionStatusCode {
    Initiated,
    FinancialInstitutionSelected
}

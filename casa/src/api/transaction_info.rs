use domain::*;
use chrono::prelude::*;

#[derive(Serialize, Debug, Clone)]
pub struct TransactionInfo {
    pub value: i64,
    pub balance: i64,
    pub transaction_time: DateTime<Utc>,
    pub business_ends: BusinessEnds,
    pub currency_code: CurrencyCode,
    pub note: String
}

impl TransactionInfo {
    pub fn from_to_transaction(tx: &Transaction) -> Self {
        Self {
            value: tx.value.to_cents(),
            balance: tx.balance_to_account,
            transaction_time: tx.transaction_time.clone(), 
            business_ends: BusinessEnds::WalletDeposit,
            currency_code: CurrencyCode::NZD,
            note: format!("Wallet deposit using credit card ($NZD{})", tx.value)
        }
    }

    pub fn from_from_transaction(tx: &Transaction) -> Self {
        Self {
            value: -tx.value.to_cents(),
            balance: tx.balance_from_account,
            transaction_time: tx.transaction_time.clone(), 
            business_ends: BusinessEnds::WalletDeposit,
            currency_code: CurrencyCode::NZD,
            note: format!("Wallet deposit using credit card ($NZD{})", tx.value)
        }
    }
}

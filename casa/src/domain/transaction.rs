use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use db::TryFromRow;
use db::TryFromRowError;
use domain::{AssetType, BusinessEnds, Id, TransactionId};
use postgres::rows::Row;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub from_account: Id,
    pub to_account: Id,
    pub asset_type: AssetType,
    pub value: i64,
    pub transaction_time: DateTime<Utc>,
    pub accounting_period_id: Id,
    pub balance: i64,
    pub message: String,
    pub business_ends: BusinessEnds,
}

impl Transaction {
    pub fn waller_to_hold() -> Self {
        unimplemented!()
    }

    pub fn hold_reversal() -> Self {
        unimplemented!()
    }

    pub fn hold_to_wallet() -> Self {
        unimplemented!()
    }

    pub fn fee_transfer() -> Self {
        unimplemented!()
    }

    pub fn wallet_deposit() -> Self {
        unimplemented!()
    }

    pub fn wallet_withdrawal() -> Self {
        unimplemented!()
    }
}

impl TryFromRow for Transaction {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let transaction_id_match: Option<TransactionId> = row.get("journal_entry_id");
        let transaction_id: TransactionId = try!(transaction_id_match.ok_or(
            TryFromRowError::missing_field("Transaction", "journal_entry_id",)
        ));

        let asset_type_match: Option<AssetType> = row.get("asset_type");
        let asset_type: AssetType = try!(
            asset_type_match.ok_or(TryFromRowError::missing_field("Transaction", "asset_type"))
        );
        let credit_match: Option<i64> = row.get("credit");
        let debit_match: Option<i64> = row.get("debit");

        let from_id_option: Option<Id> = row.get("account_id");
        let to_id_option: Option<Id> = row.get("to_account_id");

        let (from_id, to_id) = match (from_id_option, to_id_option) {
            (Some(f), Some(t)) => (f, t),
            _ => return Err(TryFromRowError::missing_field("Transaction", "account")),
        };

        let transaction_time_match: Option<NaiveDateTime> = row.get("transaction_time");
        let transaction_time: NaiveDateTime = try!(transaction_time_match.ok_or(
            TryFromRowError::missing_field("Transaction", "transaction_time"),
        ));
        let accounting_period_match: Option<Id> = row.get("accounting_period");
        let accounting_period: Id = try!(accounting_period_match.ok_or(
            TryFromRowError::missing_field("Transaction", "accounting_period",),
        ));
        let balance_match: Option<i64> = row.get("balance");
        let balance: i64 =
            try!(balance_match.ok_or(TryFromRowError::missing_field("Transaction", "balance",)));

        let message_match: Option<String> = row.get("message");
        let message: String =
            try!(message_match.ok_or(TryFromRowError::missing_field("Transaction", "message",)));

        let business_ends_match: Option<BusinessEnds> = row.get("business_ends");
        let business_ends: BusinessEnds = try!(business_ends_match.ok_or(
            TryFromRowError::missing_field("Transaction", "business_ends",)
        ));

        let value = match (credit_match, debit_match) {
            (Some(credit), None) => credit,
            (None, Some(debit)) => -debit,
            _ => {
                return Err(TryFromRowError::new(&format!(
                    "Only one of Transaction 'credit' and 'debit' fields must non-null. Got credit={:?} and debit={:?}",
                    debit_match,
                    credit_match
                )))
            }
        };

        Ok(Transaction {
            id: transaction_id,
            from_account: from_id,
            to_account: to_id,
            asset_type: asset_type,
            value: value,
            transaction_time: DateTime::from_utc(transaction_time, Utc),
            accounting_period_id: accounting_period,
            balance: balance,
            message: message,
            business_ends: business_ends,
        })
    }
}

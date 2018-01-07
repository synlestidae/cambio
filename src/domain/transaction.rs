use chrono::prelude::*;
use chrono::prelude::{DateTime, Utc};
use domain::{Id, Denom, BusinessEnds, AssetType};
use postgres::rows::Row;
use db::TryFromRow;
use db::TryFromRowError;

pub struct Transaction {
    pub id: Id,
    pub other_party: Option<Id>,
    pub asset_type: AssetType,
    pub asset_denom: Denom,
    pub value: i64,
    pub transaction_time: DateTime<Utc>,
    pub accounting_period_id: i32,
    pub balance: i64,
    pub message: String,
    pub business_ends: BusinessEnds
}

impl TryFromRow for Transaction {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let error = TryFromRowError {};

        let transaction_id_match: Option<i32> = row.get("journal.id");
        let transaction_id: i32 = try!(transaction_id_match.ok_or(error));

        let other_party: Option<i32> = None;
        let asset_type: AssetType = try!(AssetType::try_from_row(row));
        let credit_match: Option<i64> = row.get("credit");
        let debit_match: Option<i64> = row.get("debit");
        let transaction_time_match: Option<NaiveDateTime> = row.get("transaction_time");
        let transaction_time: NaiveDateTime = try!(transaction_time_match.ok_or(error));
        let accounting_period_match: Option<i32> = row.get("accounting_period");
        let accounting_period: i32 = try!(accounting_period_match.ok_or(error));
        let balance_match: Option<i64> = row.get("balance");
        let balance: i64 = try!(balance_match.ok_or(error));

        let message_match: Option<String> = row.get("message");
        let message: String = try!(message_match.ok_or(error));

        let denom = try!(Denom::try_from_row(row));//row.get("denom");
        let business_ends: BusinessEnds = try!(BusinessEnds::try_from_row(row));

        let value = match (credit_match, debit_match) {
            (Some(credit), None) => credit,
            (None, Some(debit)) => -debit,
            _ => return Err(TryFromRowError {})
        };

        Ok(Transaction {
            id: transaction_id,
            other_party: None,
            asset_type: asset_type,
            asset_denom: denom,
            value: value,
            transaction_time: DateTime::from_utc(transaction_time, Utc),
            accounting_period_id: accounting_period,
            balance: balance,
            message: message,
            business_ends: business_ends 
        })
    }
}

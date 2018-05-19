use std::fmt;
use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "account_business_type")]
pub enum AccountBusinessType {
    #[postgres(name = "user_cash_wallet")] UserCashWallet,
    #[postgres(name = "order_payment_hold")] OrderPaymentHold,
    #[postgres(name = "system_fees_paid")] SystemFeesPaid,
    #[postgres(name = "accounting_concept")] AccountingConcept,
}

impl fmt::Display for AccountBusinessType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &AccountBusinessType::UserCashWallet => "user_cash_wallet",
            &AccountBusinessType::OrderPaymentHold => "order_payment_hold",
            &AccountBusinessType::SystemFeesPaid => "system_fees_paid",
            &AccountBusinessType::AccountingConcept => "accounting_concept",
        };
        write!(f, "{}", string)
    }
}

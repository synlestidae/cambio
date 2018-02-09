use std::fmt;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "business_ends_type")]
pub enum BusinessEnds {
    #[postgres(name = "wallet_deposit")]
    WalletDeposit,
    #[postgres(name = "wallet_withdrawal")]
    WalletWithdrawal,
    #[postgres(name = "system_fee_charge")]
    SystemFeeCharge,
    //#[postgres(name = "cryptocurrency_purchase")]
    //CryptocurrencyPurchase,
    #[postgres(name = "order_placement")]
    OrderPlacement,
    #[postgres(name = "order_settlement")]
    OrderSettlement
}

impl fmt::Display for BusinessEnds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            &BusinessEnds::WalletDeposit => "wallet_deposit",
            &BusinessEnds::WalletWithdrawal => "wallet_withdrawal",
            &BusinessEnds::SystemFeeCharge => "system_fee_charge",
            //&BusinessEnds::CryptocurrencyPurchase => "cryptocurrency_purchase",
            &BusinessEnds::OrderPlacement => "order_placement",
            &BusinessEnds::OrderSettlement => "order_settlement"
        };
        write!(f, "{}", as_string)
    }
}

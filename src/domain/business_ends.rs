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
    #[postgres(name = "cryptocurrency_purchase")]
    CryptocurrencyPurchase,
}

impl BusinessEnds {
    pub fn parse(business_ends: &str) -> Option<BusinessEnds> {
        match business_ends {
            "wallet_deposit" => Some(BusinessEnds::WalletDeposit),
            "wallet_withdrawal" => Some(BusinessEnds::WalletWithdrawal),
            "system_fee_charge" => Some(BusinessEnds::SystemFeeCharge),
            "cryptocurrency_purchase" => Some(BusinessEnds::CryptocurrencyPurchase),
            _ => None,
        }
    }
}

impl TryFromRow for BusinessEnds {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let business_ends_string: Option<String> = row.get("business_ends");
        if business_ends_string.is_none() {
            return Err(TryFromRowError {});
        }
        match BusinessEnds::parse(&business_ends_string.unwrap()) {
            Some(business_ends) => Ok(business_ends),
            None => Err(TryFromRowError {}),
        }
    }
}

impl fmt::Display for BusinessEnds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_string = match self {
            &BusinessEnds::WalletDeposit => "wallet_deposit",
            &BusinessEnds::WalletWithdrawal => "wallet_withdrawal",
            &BusinessEnds::SystemFeeCharge => "system_fee_charge",
            &BusinessEnds::CryptocurrencyPurchase => "cryptocurrency_purchase",
        };
        write!(f, "{}", as_string)
    }
}

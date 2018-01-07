use std::fmt;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

pub enum BusinessEnds {
    WalletDeposit,
    WalletWithdrawal,
    SystemFeeCharge,
    CryptocurrencyPurchase
}

impl BusinessEnds {
    pub fn parse(business_ends: &str) -> Option<BusinessEnds> {
        match business_ends {
            "wallet_deposit" => Some(BusinessEnds::WalletDeposit),
            "wallet_withdrawal" => Some(BusinessEnds::WalletWithdrawal),
            "system_fee_charge" => Some(BusinessEnds::SystemFeeCharge),
            "cryptocurrency_purchase" => Some(BusinessEnds::CryptocurrencyPurchase),
            _ => None
        }
    }
}

impl TryFromRow for BusinessEnds {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let business_ends_string:Option<String> = row.get("business_ends");
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
            WalletDeposit => "wallet_deposit",
            WalletWithdrawal => "wallet_withdrawal",
            SystemFeeCharge => "system_fee_charge",
            CryptocurrencyPurchase => "cryptocurrency_purchase"
        };
        write!(f, "{}", as_string)
    }
}

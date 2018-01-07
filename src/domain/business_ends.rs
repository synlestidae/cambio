use std::fmt;

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

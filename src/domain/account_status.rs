use std::fmt;

pub enum AccountStatus {
    Active,
    Frozen,
    Closed
}

impl AccountStatus {
    pub fn parse(business_ends: &str) -> Option<AccountStatus> {
        match business_ends {
            "active" => Some(AccountStatus::Active),
            "frozen" => Some(AccountStatus::Frozen),
            "closed" => Some(AccountStatus::Closed),
            _ => None
        }
    }
}

impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            &AccountStatus::Active => "active",
            &AccountStatus::Frozen=> "frozen",
            &AccountStatus::Closed => "closed"
        };
        write!(f, "{}", string)
    }
}

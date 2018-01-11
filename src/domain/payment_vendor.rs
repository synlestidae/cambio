use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
#[postgres(name = "payment_vendor")]
pub enum PaymentVendor {
    #[postgres(name = "Poli")]
    Poli
}

impl fmt::Display for PaymentVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match self {
            &PaymentVendor::Poli => "Poli",
        };
        write!(f, "{}", display)
    }
}

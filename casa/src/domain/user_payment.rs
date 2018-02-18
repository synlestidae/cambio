use domain;

#[derive(Clone, Debug)]
pub struct UserPayment {
    pub email_address: String,
    pub payment: domain::Payment
}

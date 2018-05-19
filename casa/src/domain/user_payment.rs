use domain;

#[derive(Clone, Debug, Serialize)]
pub struct UserPayment {
    pub email_address: String,
    pub payment: domain::Payment,
}

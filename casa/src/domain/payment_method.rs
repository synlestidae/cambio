#[derive(Debug, Clone, PartialEq, Eq, ToSql, FromSql, Deserialize, Serialize)]
                //let mut api = AccountApiImpl::new(this_helper_ref.clone());
                //Ok(api.get_accounts(r))
#[postgres(name = "payment_method")]
pub enum PaymentMethod {
    #[postgres(name = "nz_bank_deposit")]
    NZBankDeposit,
    #[postgres(name = "credit_card")]
    CreditCard,
}

use api::CryptoAccountRequest;

#[derive(Debug)]
pub enum CryptoAccountApiRequest {
    GetAccounts,
    NewAccount(CryptoAccountRequest),
    ModifyAccount(CryptoAccountRequest),
}

use domain::ByteAddress;
use domain::CryptoType;
use domain::EthAccountId;

#[derive(Deserialize, Debug, Clone)]
pub struct CryptoAccountRequest {
    pub id: Option<EthAccountId>,
    pub address: ByteAddress,
    pub name: String,
    pub currency_type: CryptoType
}

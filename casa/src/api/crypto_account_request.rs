use domain::ByteAddress;
use domain::CryptoType;
use domain::Id;

#[derive(Deserialize, Debug)]
pub struct CryptoAccountRequest {
    pub id: Option<Id>,
    pub address: ByteAddress,
    pub name: String,
    pub currency_type: CryptoType
}

use serde::{Deserialize, Deserializer};

#[derive(Serialize, Debug, Clone)]
pub struct MerchantCode(pub String);

impl<'de> Deserialize<'de> for MerchantCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = try!(String::deserialize(deserializer));
        Ok(MerchantCode(data))
    }
}

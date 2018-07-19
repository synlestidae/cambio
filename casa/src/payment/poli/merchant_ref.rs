use serde::{Deserialize, Deserializer};

#[derive(Serialize, Debug)]
pub struct MerchantRef(pub String);

impl<'de> Deserialize<'de> for MerchantRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = try!(String::deserialize(deserializer));
        Ok(MerchantRef(data))
    }
}

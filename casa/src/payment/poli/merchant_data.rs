use serde::{Deserialize, Deserializer};

#[derive(Serialize)]
pub struct MerchantData(pub String);

impl<'de> Deserialize<'de> for MerchantData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = try!(String::deserialize(deserializer));
        Ok(MerchantData(data))
    }
}

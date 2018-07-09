use serde::{Deserialize, Deserializer};

#[derive(Serialize, Debug, Clone)]
pub struct AuthenticationCode(pub String);

impl<'de> Deserialize<'de> for AuthenticationCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let data = try!(String::deserialize(deserializer));
        Ok(AuthenticationCode(data))
    }
}

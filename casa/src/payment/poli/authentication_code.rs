use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Serialize, Debug, Clone)]
pub struct AuthenticationCode(pub String);

impl<'de> Deserialize<'de> for AuthenticationCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = try!(String::deserialize(deserializer));
        Ok(AuthenticationCode(data))
    }
}

impl fmt::Display for AuthenticationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

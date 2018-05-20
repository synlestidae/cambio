use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSql, FromSql)]
pub struct SessionToken(pub String);

impl fmt::Display for SessionToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

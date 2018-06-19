#[derive(Eq, PartialEq, ToSql, FromSql, Debug, Clone, Deserialize, Serialize)]
pub struct IdentifierCode(pub String);

pub enum Value {
    Str(String),
    Signed32(i32),
    Unsigned32(u32)
}

impl Value {
    pub fn to_sql_value(&self) -> String {
        match self {
            &Value::Str(ref string) => format!("'{}'", string),
            &Value::Signed32(ref num) => num.to_string(),
            &Value::Unsigned32(ref num) => num.to_string()
        }
    }
}

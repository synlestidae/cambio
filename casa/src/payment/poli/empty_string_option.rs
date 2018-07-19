use serde;
use serde::de::*;
use serde::Deserialize;
use serde_json;
use serde_json::Value;
use std::error::Error as StdError;
use serde::de::Error;

pub fn deserialize<'de, D, E: DeserializeOwned>(deserializer: D) -> Result<Option<E>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_input_option: serde_json::Value = Value::deserialize(deserializer)?;
    let string_input = match string_input_option {
        Value::Null => return Ok(None),
        Value::String(s) => {
            if s.len() == 0 {
                return Ok(None);
            }
            Value::String(s)
        }
        Value::Number(n) => Value::String(n.to_string()),
        the_rest => the_rest,
    };

    println!("JSON {:?}", string_input);
    match serde_json::from_value(string_input) {
        Ok(o) => Ok(Some(o)),
        Err(err) => {
            println!("SHit error {:?}", err);
            Err(D::Error::custom(StdError::description(&err)))
        }
    }
}

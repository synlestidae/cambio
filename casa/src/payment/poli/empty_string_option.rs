use serde::de::*;
use serde;
use serde_json;
use serde::Deserialize;

pub fn deserialize<'de, D, E: DeserializeOwned>(deserializer: D) -> Result<Option<E>, D::Error>
    where
        D: Deserializer<'de>,
{
    let string_input_option: Option<String> = Option::deserialize(deserializer)?; 
    let string_input = match string_input_option {
        Some(s) => s,
        None => return Ok(None)
    };
    if string_input.len() == 0 {
        Ok(None)
    } else {
        match serde_json::to_string(&string_input) {
            Ok(s) => {
                match serde_json::from_str(&s) {
                    Ok(o) => return Ok(Some(o)),
                    Err(err) => return unimplemented!()
                }
            },
            Err(err) => return unimplemented!()
        }
    }
}

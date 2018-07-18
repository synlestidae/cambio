use chrono::prelude::*;
use serde::de::*;
use serde;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
{
    let string_option: Option<String> = Option::deserialize(deserializer)?; 
    if let Some(s) = string_option {
        if s.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(parse::<D>(&s)?))
        }
    } else {
        Ok(None)
    }
}

fn parse<'a, D: Deserializer<'a>>(date_string: &str) -> Result<NaiveDateTime, D::Error> {
    const FORMAT: &'static str = "%Y-%m-%dT%H-%M-%S%.3f";
    Utc.datetime_from_str(date_string, FORMAT).map_err(serde::de::Error::custom).map(|x| x.naive_utc())
}

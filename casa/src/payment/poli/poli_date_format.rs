use chrono::prelude::*;
use serde;
use serde::de::*;

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f";
    let date_string = String::deserialize(deserializer)?;
    println!("Datey {}", date_string);
    Utc.datetime_from_str(&date_string, FORMAT)
        .map_err(serde::de::Error::custom)
        .map(|x| x.naive_utc())
}

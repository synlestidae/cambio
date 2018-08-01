use chrono::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct LastChange {
    #[serde(with = "last_change_date_format")]
    pub last_change: DateTime<Utc>
}

mod last_change_date_format {
    use chrono::prelude::*;
    use serde;
    use serde::de::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FORMAT: &'static str = "%Y%m%d%H%M%S%.3f";
        let date_string = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&date_string, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

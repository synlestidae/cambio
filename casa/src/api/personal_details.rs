use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalDetails {
    pub first_names: String,
    pub family_name: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub post_code: String,
    pub city: String,
    pub country: String,
    pub dob: DateTime<Utc>, 
    pub id_type: String,
    pub id_number: String
}

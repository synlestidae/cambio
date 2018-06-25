use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalDetails {
    first_names: String,
    family_name: String,
    address_line_1: String,
    address_line_2: String,
    post_code: String,
    city: String,
    country: String,
    dob: NaiveDate,
    id_type: String,
    id_number: String
}

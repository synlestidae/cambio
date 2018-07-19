use chrono::prelude::*;
use domain::{Address, PersonalIdentity, Profile, UserId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalDetails {
    pub first_names: String,
    pub family_name: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub post_code: String,
    pub city: String,
    pub country: String,
    pub dob: NaiveDate,
    pub id_type: String,
    pub id_number: String,
}

impl PersonalDetails {
    pub fn into_profile(self, user_id: UserId) -> Profile {
        Profile {
            id: None,
            user_id: user_id,
            given_names: self.first_names,
            family_names: self.family_name,
            date_of_birth: self.dob,
            address: Address {
                id: None,
                address_line_1: Some(self.address_line_1),
                address_line_2: self.address_line_2,
                address_line_3: Some(self.post_code),
                address_line_4: Some(self.city),
                address_line_5: None,
                address_line_6: None,
                address_line_7: None,
                country_name: self.country,
            },
            personal_identity: None,
        }
    }
}

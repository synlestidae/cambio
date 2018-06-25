use chrono::NaiveDate;
use domain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    given_names: String,
    family_names: String,
    date_of_birth: NaiveDate,
    contact_details: domain::ContactInfo,
    personal_identity: domain::PersonalIdentity,
}

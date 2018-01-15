use domain;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    complete_name: String,
    date_of_birth: NaiveDate,
    contact_details: domain::ContactInfo,
    personal_identity: domain::PersonalIdentity,
}

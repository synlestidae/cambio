use chrono::NaiveDate;
use domain;
use db::TryFromRow;
use db::TryFromRowError;
use postgres::rows::Row;

#[derive(Debug, Clone)]
pub struct Profile {
    given_names: String,
    family_names: String,
    date_of_birth: NaiveDate,
    contact_details: domain::ContactInfo,
    personal_identity: Option<domain::PersonalIdentity>
}

use domain::{Id, Address, ContactInfo, PersonalIdentity}; 
use chrono::prelude::*;

#[derive(Debug, Clone, TryFromRow)]
pub struct PersonalInfo {
    id: Id, 
    given_names: String, 
    family_names: String, 
    date_of_birth: NaiveDate,
    address_id: Address,
    contact_info_id: ContactInfo,
    personal_identity: Option<PersonalIdentity>
}

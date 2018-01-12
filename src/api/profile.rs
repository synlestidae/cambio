use domain;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    contact_details: domain::ContactInfo,
    personal_identity: domain::PersonalIdentity,
}

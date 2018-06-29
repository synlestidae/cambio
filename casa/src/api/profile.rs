use chrono::NaiveDate;
use domain;
use db::CambioError;
use db::TryFromRow;
use db::TryFromRowError;
use postgres::rows::Row;

#[derive(Debug, Clone, TryFromRow)]
pub struct Profile {
    id: domain::Id,
    given_names: String,
    family_names: String,
    date_of_birth: NaiveDate,
    contact_details: domain::ContactInfo,
    address: domain::Address,
    personal_identity: Option<domain::PersonalIdentity>
}

impl TryFromRow for Profile {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let user_id = row.get("user_id").ok_or(TryFromRowError::missing_field("personal_info", "user_id"));
        let given_names = row.get("given_names").ok_or(TryFromRowError::missing_field("personal_info", "given_names"));
        let family_names = row.get("family_names").ok_or(TryFromRowError::missing_field("personal_info", "family_names"));
        let date_of_birth = row.get("date_of_birth").ok_or(TryFromRowError::missing_field("personal_info", "date_of_birth"));
    }
}

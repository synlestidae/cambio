use chrono::NaiveDate;
use domain;
use db::TryFromRow;
use db::TryFromRowError;
use postgres::rows::Row;

#[derive(Debug, Clone)]
pub struct Profile {
    pub id: Option<domain::ProfileId>,
    pub user_id: domain::UserId,
    pub given_names: String,
    pub family_names: String,
    pub date_of_birth: NaiveDate,
    pub address: domain::Address,
    pub personal_identity: Option<domain::PersonalIdentity>
}

use domain::Id;
use postgres::rows::Row;
use db::{get_value, get_value_option, TryFromRow, TryFromRowError};
use postgres;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TryFromRow)]
pub struct ContactInfo {
    #[column_id(contact_info_id)]
    id: Option<Id>,
    primary_email: String,
    backup_email: Option<String>,
    main_intl_phone_number: Option<String>,
    secondary_intl_phone_number: Option<String>,
}

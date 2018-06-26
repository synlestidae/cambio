use db::{get_value, get_value_option, TryFromRow, TryFromRowError};
use domain::Id;
use postgres;
use postgres::rows::Row;

#[derive(Debug, Clone, TryFromRow)]
pub struct ContactInfo {
    #[column_id(contact_info_id)]
    id: Option<Id>,
    primary_email: String,
    backup_email: Option<String>,
    main_intl_phone_number: Option<String>,
    secondary_intl_phone_number: Option<String>,
}

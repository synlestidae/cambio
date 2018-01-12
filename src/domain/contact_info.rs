use domain::Id;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError, get_value, get_value_option};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactInfo {
    id: Option<Id>,
    primary_email: String,
    backup_email: Option<String>,
    main_intl_phone_number: Option<String>,
    secondary_intl_phone_number: Option<String>,
}

impl TryFromRow for ContactInfo {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let id: Option<i32> = row.get("contact_info_id"); // remember, id has different name
        let primary_email = try!(get_value("Account", "primary_email", row)); 
        let backup_email= try!(get_value_option("Account", "backup_email", row)); 
        let main_intl_phone_number = try!(get_value_option("Account", "main_intl_phone_number", row)); 
        let secondary_intl_phone_number = try!(get_value_option("Account", "secondary_intl_phone_number", row)); 

        Ok(ContactInfo {
            id: id,
            primary_email: primary_email,
            backup_email: backup_email,
            main_intl_phone_number: main_intl_phone_number,
            secondary_intl_phone_number:secondary_intl_phone_number
        })
    }
}

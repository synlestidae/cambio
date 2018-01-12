use domain::Id;
use postgres::rows::Row;
use db::{TryFromRow, TryFromRowError};

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
        let primary_email = try!(get_string("Account", "primary_email", row)); 
        let backup_email= try!(get_string_option("Account", "backup_email", row)); 
        let main_intl_phone_number = try!(get_string_option("Account", "main_intl_phone_number", row)); 
        let secondary_intl_phone_number = try!(get_string_option("Account", "secondary_intl_phone_number", row)); 

        Ok(ContactInfo {
            id: id,
            primary_email: primary_email,
            backup_email: backup_email,
            main_intl_phone_number: main_intl_phone_number,
            secondary_intl_phone_number:secondary_intl_phone_number
        })
    }
}

fn get_string<'a>(entity: &str, field: &str, row: &Row<'a>) -> Result<String, TryFromRowError> {
    match get_string_option(entity, field, row) {
        Ok(None) => Err(TryFromRowError::missing_field(entity, field)),
        Ok(Some(string)) => Ok(string),
        Err(error) => Err(error),
    }
}

fn get_string_option<'a>(entity: &str, field: &str, row: &Row<'a>) -> Result<Option<String>, TryFromRowError> {
    match row.get_opt(field) {
        Some(Ok(string)) => Ok(string),
        None => Err(TryFromRowError::missing_field(entity, field)),
        Some(Err(err)) => Err(TryFromRowError::new(&format!("Error getting field '{}' for {}: {}", field, entity, err)))
    }
}

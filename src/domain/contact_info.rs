use domain::Id;

pub struct ContactInfo {
    id: Option<Id>,
    primary_email: Option<String>,
    backup_email: Option<String>,
    main_intl_phone_number: Option<String>,
    secondary_intl_phone_number: Option<String>,
}

use domain::Id;

pub struct Address {
    id: Option<Id>, 
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    address_line_3: Option<String>,
    address_line_4: Option<String>,
    address_line_5: Option<String>,
    address_line_6: Option<String>,
    address_line_7: Option<String>,
    country_iso3: String
}

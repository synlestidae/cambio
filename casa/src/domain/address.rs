use domain::Id;

#[Derive(Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Address {
    #[column_id(address_id)]
    id: Option<Id>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    address_line_3: Option<String>,
    address_line_4: Option<String>,
    address_line_5: Option<String>,
    address_line_6: Option<String>,
    address_line_7: Option<String>,
    country_iso3: String,
}

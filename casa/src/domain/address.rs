use domain::Id;
use postgres;
use postgres::rows::Row;
use db::TryFromRow;
use db::TryFromRowError;

#[derive(Debug, Clone, PartialEq, Eq, TryFromRow)]
pub struct Address {
    pub id: Option<Id>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub address_line_3: Option<String>,
    pub address_line_4: Option<String>,
    pub address_line_5: Option<String>,
    pub address_line_6: Option<String>,
    pub address_line_7: Option<String>,
    pub country_name: String
}

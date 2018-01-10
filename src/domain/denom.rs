use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Denom {
    Dollar,
    Cent,
    Sat,
    Wei,
}

impl ToString for Denom {
    fn to_string(&self) -> String {
        let denom_str = match self {
            &Denom::Dollar => "dollar",
            &Denom::Cent => "cent",
            &Denom::Sat => "satoshi",
            &Denom::Wei => "wei",
        };
        denom_str.to_owned()
    }
}

impl TryFromRow for Denom {
    fn try_from_row<'a>(row: &Row<'a>) -> Result<Self, TryFromRowError> {
        let denom_str_match: Option<String> = row.get("denom");
        if denom_str_match.is_none() {
            return Err(TryFromRowError {});
        }
        match denom_str_match.unwrap().as_ref() {
            "dollar" => Ok(Denom::Dollar),
            "cent" => Ok(Denom::Cent),
            "sat" => Ok(Denom::Sat),
            "wei" => Ok(Denom::Wei),
            _ => Err(TryFromRowError {}),
        }
    }
}

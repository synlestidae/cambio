use db::{TryFromRow, TryFromRowError};
use postgres::rows::Row;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, FromSql, ToSql)]
#[postgres(name = "denom_type")]
pub enum Denom {
    #[postgres(name = "dollar")]
    Dollar,
    #[postgres(name = "cent")]
    Cent,
    #[postgres(name = "satoshi")]
    Sat,
    #[postgres(name = "wei")]
    Wei,
    #[postgres(name = "szabo")]
    Szabo,
}

impl ToString for Denom {
    fn to_string(&self) -> String {
        let denom_str = match self {
            &Denom::Dollar => "dollar",
            &Denom::Cent => "cent",
            &Denom::Sat => "satoshi",
            &Denom::Wei => "wei",
            &Denom::Szabo => "szabo",
        };
        denom_str.to_owned()
    }
}

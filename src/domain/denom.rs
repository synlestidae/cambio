use serde_derive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Denom {
    Dollar,
    Cent,
    Sat
}

impl ToString for Denom {
    fn to_string(&self) -> String {
        let denom_str = match self {
            &Denom::Dollar => "dollar",
            &Denom::Cent=> "cent",
            &Denom::Sat=> "satoshi",
        };
        denom_str.to_owned()
    }
}

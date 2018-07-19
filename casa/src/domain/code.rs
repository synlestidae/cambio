use postgres::types::{FromSql, IsNull, ToSql, Type};
use rand;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::iter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Code(String);

impl Code {
    pub fn new() -> Code {
        let mut rng = rand::thread_rng();
        Code(
            iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(12)
                .collect(),
        )
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl FromSql for Code {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<Error + 'static + Send + Sync>> {
        match String::from_utf8(raw.iter().map(|&x| x).collect()) {
            Ok(s) => Ok(Code(s)),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}

impl ToSql for Code {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut Vec<u8>,
    ) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
        out.append(&mut bytes);
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        true
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut Vec<u8>,
    ) -> Result<IsNull, Box<Error + 'static + Send + Sync>> {
        let mut bytes = self.0.clone().bytes().collect();
        out.append(&mut bytes);
        Ok(IsNull::No)
    }
}

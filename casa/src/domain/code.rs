use std::fmt::{Display, Formatter, Error};
use rand::distributions::Alphanumeric;
use std::iter;
use rand::Rng;
use rand;

#[derive(Serialize, Deserialize)]
pub struct Code(String);

impl Code {
    pub fn new() -> Code {
        let mut rng = rand::thread_rng();
        Code(iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(12)
            .collect()
        )
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

use std::error;
use std::error::Error;
use std::fmt;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct TryFromRowError {
    desc: String,
}

impl TryFromRowError {
    pub fn new(desc: &str) -> Self {
        TryFromRowError {
            desc: desc.to_owned(),
        }
    }

    pub fn missing_field(entity: &str, name: &str) -> Self {
        TryFromRowError {
            desc: format!("Entity '{}' is missing required field : '{}'", entity, name),
        }
    }

    pub fn bad_value<T: fmt::Display>(entity: &str, name: &str, val: T) -> Self {
        TryFromRowError {
            desc: format!(
                "Field '{}' on entity '{}' has invalid value: {}",
                name, entity, val
            ),
        }
    }

    pub fn unknown_value(entity: &str, value: &str) -> Self {
        TryFromRowError {
            desc: format!("Unknown value for '{}': {}", entity, value),
        }
    }
}

impl error::Error for TryFromRowError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl fmt::Display for TryFromRowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.description())
    }
}

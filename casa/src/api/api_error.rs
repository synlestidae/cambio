use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    desc: String,
    error_type: ErrorType,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error of type '{:?}': {}", self.error_type, self.desc)
    }
}

impl ApiError {
    pub fn new(description: String, err_type: ErrorType) -> Self {
        ApiError {
            desc: description,
            error_type: err_type,
        }
    }

    pub fn err_type(&self) -> ErrorType {
        self.error_type
    }

    pub fn not_logged_in(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::NotLoggedIn)
    }

    pub fn invalid_login(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::InvalidLogin)
    }

    pub fn database_driver(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::DatabaseDriver)
    }

    pub fn bad_format(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::BadFormat)
    }

    pub fn missing_field_or_param(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::MissingFieldOrParam)
    }

    pub fn query_result_format(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::QueryResultFormat)
    }

    pub fn unknown(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::Unknown)
    }

    pub fn internal(description: &str) -> Self {
        Self::new(description.to_owned(), ErrorType::InternalError)
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        return &self.desc;
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ErrorType {
    DatabaseDriver,
    NotLoggedIn,
    InvalidLogin,
    BadFormat,
    MissingFieldOrParam,
    QueryResultFormat,
    InternalError,
    Unknown
}

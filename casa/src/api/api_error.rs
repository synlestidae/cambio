use iron::status::Status;
use hyper::mime::Mime;
use iron::{Response};
use iron;
use serde_json;
use std::convert::Into;
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

impl Into<Response> for ApiError {
    fn into(self) -> Response {
        let status: Status = self.error_type.into();
        let response_json = serde_json::to_string(&self).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        iron::Response::with((iron::status::Ok, response_json, content_type))
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

impl Into<Status> for ErrorType {
    fn into(self) -> Status {
        match self {
            ErrorType::DatabaseDriver => Status::InternalServerError,
            ErrorType::NotLoggedIn => Status::Unauthorized,
            ErrorType::InvalidLogin => Status::Unauthorized,
            ErrorType::BadFormat => Status::BadRequest,
            ErrorType::MissingFieldOrParam => Status::BadRequest,
            ErrorType::QueryResultFormat => Status::InternalServerError,
            ErrorType::InternalError => Status::InternalServerError,
            ErrorType::Unknown => Status::InternalServerError
        }
    }
}

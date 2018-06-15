use db;
use db::CambioError;
use hyper::mime::Mime;
use iron;
use iron::status::Status;
use iron::Response;
use serde_json;
use std::convert::{From, Into};
use std::error::Error;
use std::fmt;
use hyper::method::Method;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    desc: String,
    error_type: ErrorType,
    error: Option<db::CambioError>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error of type '{:?}': {}", self.error_type, self.desc)
    }
}

#[allow(dead_code)]
impl ApiError {
    pub fn new(description: String, err_type: ErrorType) -> Self {
        ApiError {
            desc: description,
            error_type: err_type,
            error: None,
        }
    }

    pub fn cambio_error(description: String, err_type: ErrorType, cambio_err: CambioError) -> Self {
        ApiError {
            desc: description,
            error_type: err_type,
            error: Some(cambio_err),
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

    pub fn bad_method(supported_method: Method) -> Self {
        let msg = format!("Incorrect HTTP method for this resource. Supported method is {}", 
            supported_method);
        Self::new(msg, ErrorType::BadMethod)
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

    pub fn not_found(object_name: &str) -> Self {
        Self::new(format!("{} not found.", object_name), ErrorType::NotFound)
    }

    pub fn not_found_path(path: &str) -> Self {
        Self::new(format!("The API path '{}' does not exist.", path), ErrorType::NotFound)
    }

    pub fn unauthorised() -> Self {
        Self::new(
            format!("Please log in to access this."),
            ErrorType::Unauthorised,
        )
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
        iron::Response::with((iron::status::BadRequest, response_json, content_type))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ErrorType {
    BadFormat,
    BadMethod,
    DatabaseDriver,
    InternalError,
    InvalidLogin,
    MissingFieldOrParam,
    NotFound,
    NotLoggedIn,
    QueryResultFormat,
    Unauthorised,
    Unknown,
}

impl Into<Status> for ErrorType {
    fn into(self) -> Status {
        match self {
            ErrorType::BadFormat => Status::BadRequest,
            ErrorType::BadMethod=> Status::MethodNotAllowed,
            ErrorType::DatabaseDriver => Status::InternalServerError,
            ErrorType::InternalError => Status::InternalServerError,
            ErrorType::InvalidLogin => Status::Unauthorized,
            ErrorType::MissingFieldOrParam => Status::BadRequest,
            ErrorType::NotFound => Status::NotFound,
            ErrorType::NotLoggedIn => Status::Unauthorized,
            ErrorType::NotLoggedIn => Status::Unauthorized,
            ErrorType::QueryResultFormat => Status::InternalServerError,
            ErrorType::Unauthorised => Status::Unauthorized,
            ErrorType::Unknown => Status::InternalServerError,
        }
    }
}

impl From<CambioError> for ApiError {
    fn from(err: CambioError) -> Self {
        ApiError::cambio_error("An error occurred.".to_owned(), ErrorType::Unknown, err)
    }
}

use api;
use serde::Serialize;

pub type ApiResult<T: Serialize> = Result<T, api::ApiError>;

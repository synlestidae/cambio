use api;

pub type ApiResult<T> = Result<T, api::ApiError>;

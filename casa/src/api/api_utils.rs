use serde::Deserialize;
use api::ApiError;
use iron::{Request, Response};
use iron;
use hyper::mime::Mime;
use iron::prelude::*;
use serde_json;
use bodyparser;
use std::error::Error;

pub fn get_api_obj<T: Clone + 'static>(request: &mut Request) -> Result<T, Response>
where
    for<'a> T: Deserialize<'a>,
{
    let content_type = "application/json".parse::<Mime>().unwrap();
    match request.get_ref::<bodyparser::Struct<T>>() {
        Ok(&Some(ref body_obj)) => Ok(body_obj.clone()),
        Ok(&None) => {
            let error_obj = ApiError::bad_format("Body of HTTP request cannot be empty");
            let response_json = serde_json::to_string(&error_obj).unwrap();
            Err(Response::with((
                iron::status::BadRequest,
                response_json,
                content_type,
            )))
        }
        Err(error) => {
            let error_obj = ApiError::bad_format(error.description());
            let response_json = serde_json::to_string(&error_obj).unwrap();
            Err(Response::with((
                iron::status::Ok,
                response_json,
                content_type,
            )))
        }
    }
}

use iron::request::Request;
use iron::headers::{Cookie, Authorization, Bearer};
use iron;
use serde::Serialize;
use hyper::mime::{Mime};
use serde_json;
use api::ApiError;
use db::CambioError;

pub fn get_session_token(r: &Request) -> Option<String> {
    let authorization:Option<&Authorization<Bearer>> = r.headers.get();
    match authorization {
        Some(ref bearer) => return Some(bearer.token.to_owned()),
        None => {}
    }
    let cookies_match: Option<&Cookie> = r.headers.get();
    if cookies_match.is_none() {
        return None;
    }
    let cookie_header = cookies_match.unwrap();
    for cookie in cookie_header.0.iter() {
        let cookie_bits: Vec<String> = cookie.clone().split("=").map(|s| s.to_owned()).collect();
        if cookie_bits[0] == "session_token" {
            let token = cookie_bits[1].clone();
            return Some(token);
        }
    }
    None
}

pub fn to_response<E: Serialize>(result: Result<E, CambioError>) -> iron::Response {
    let content_type = "application/json".parse::<Mime>().unwrap();
    match result {
        Ok(response_obj) => {
            let response_json = serde_json::to_string(&response_obj).unwrap();
            iron::Response::with((iron::status::Ok, response_json, content_type))
        },
        Err(err) => {
            let api_error: ApiError = err.into();
            api_error.into()
        }
    }
}

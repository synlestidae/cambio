use domain::SessionToken;
use iron::headers::{Authorization, Bearer, Cookie};
use iron::request::Request;

pub trait SessionTokenSource {
    fn get_session_token(&self) -> Option<SessionToken>;
}

impl<'a, 'b> SessionTokenSource for Request<'a, 'b> {
    fn get_session_token(&self) -> Option<SessionToken> {
        let authorization: Option<&Authorization<Bearer>> = self.headers.get();
        match authorization {
            Some(ref bearer) => return Some(SessionToken(bearer.token.to_owned())),
            None => {}
        }
        let cookies_match: Option<&Cookie> = self.headers.get();
        if cookies_match.is_none() {
            return None;
        }
        let cookie_header = cookies_match.unwrap();
        for cookie in cookie_header.0.iter() {
            let cookie_bits: Vec<String> =
                cookie.clone().split("=").map(|s| s.to_owned()).collect();
            if cookie_bits[0] == "session_token" {
                let token = cookie_bits[1].clone();
                return Some(SessionToken(token));
            }
        }
        None
    }
}

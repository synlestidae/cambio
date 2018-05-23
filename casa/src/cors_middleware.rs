use iron::headers;
use iron::method::Method;
use iron::middleware::Handler;
use iron::prelude::*;
use iron::status::Status;
use iron::AroundMiddleware;

pub struct CorsMiddleware {}

impl AroundMiddleware for CorsMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        let new_handler = CorsHandler { handler: handler };
        Box::new(new_handler)
    }
}

pub struct CorsHandler {
    handler: Box<Handler>,
}

impl Handler for CorsHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let methods = vec![Method::Get, Method::Post, Method::Put, Method::Delete];
        let mut response = if req.method == Method::Options {
            let mut r = Response::new();
            r.status = Some(Status::Ok);
            r
        } else {
            try!(self.handler.handle(req))
        };
        response
            .headers
            .set(headers::AccessControlAllowMethods(methods));
        response.headers.set(headers::AccessControlAllowCredentials);
        response
            .headers
            .set(headers::AccessControlAllowOrigin::Value(
                "http://localhost:8080".to_owned(),
            ));
        let request_headers: Option<headers::AccessControlRequestHeaders> = req
            .headers
            .get()
            .map(|h: &headers::AccessControlRequestHeaders| h.clone());
        match request_headers {
            Some(hs) => {
                response
                    .headers
                    .set(headers::AccessControlAllowHeaders(hs.0));
            }
            None => {}
        };
        Ok(response)
    }
}

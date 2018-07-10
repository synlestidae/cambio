#[derive(Debug)]
pub enum PoliError {
    HTTPRequest(Box<Error>),
    HTTPResponse(Box<Error),
    PoliErrorCode(PoliErrorCode)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    DBConnection,
    GethConnection,
    Query,
    Unauthorised,
    ConvertingObjInternal,
    UserInputFormat,
    UserExists,
    InsufficientFunds,
    NotFound
}

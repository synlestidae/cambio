#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ErrorKind {
    DBConnection,
    GethConnection,
    Web3,
    Query,
    Unauthorised,
    ConvertingObjInternal,
    FormatObjInternal,
    UserInputFormat,
    UserExists,
    InsufficientFunds,
    NotFound,
    UnexpectedState,
    UnfairOperation
}

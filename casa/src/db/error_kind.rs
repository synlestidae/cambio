#[derive(Debug, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
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
    UnfairOperation,
    StateChangeNotPermitted,
    OverUserLimit,
}

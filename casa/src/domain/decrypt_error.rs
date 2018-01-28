use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum DecryptError {
    FromUtf8Error(FromUtf8Error),
    DecryptedDataHashError
}

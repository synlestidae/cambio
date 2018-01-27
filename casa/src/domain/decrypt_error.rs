use std::string::FromUtf8Error;
use crypto::symmetriccipher::SymmetricCipherError;

#[derive(Debug)]
pub enum DecryptError {
    FromUtf8Error(FromUtf8Error),
    SymmetricCipherError(SymmetricCipherError),
    DecryptedDataHashError
}
